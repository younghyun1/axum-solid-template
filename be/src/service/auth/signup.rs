use std::sync::Arc;

use chrono::{Duration, Utc};
use email_address::EmailAddress;
use uuid::Uuid;
use zeroize::Zeroize;

use crate::{
    domain::auth::{role::RoleType, user::NewEmailVerificationToken, user::NewUser},
    dto::{
        api_response::ApiResult,
        auth::{request::SignupRequest, response::SignupResponse},
    },
    error::{api_error::ApiError, code_error::CodeError},
    init::state::server_state::ServerState,
    repository::auth::postgres::{
        email_verification_token_repository,
        user_repository::{self, InsertUserError},
        user_role_repository,
    },
    service::auth::{datasource::postgres_conn, email::queue_verification_email},
    util::{
        crypto::password::hash_password,
        string::validation::{normalized_email, validate_password_form, validate_username},
    },
};

const EMAIL_VERIFICATION_TOKEN_VALID_DURATION: Duration = Duration::days(1);

pub async fn signup_user(
    state: Arc<ServerState>,
    mut request: SignupRequest,
) -> ApiResult<SignupResponse> {
    if !validate_username(&request.user_name) {
        return Err(ApiError::new(CodeError::USER_NAME_INVALID));
    }
    if !EmailAddress::is_valid(&request.user_email) {
        return Err(ApiError::new(CodeError::EMAIL_INVALID));
    }
    if !validate_password_form(&request.user_password) {
        return Err(ApiError::new(CodeError::PASSWORD_INVALID));
    }

    let user_email = normalized_email(&request.user_email);
    let password_hash = match hash_password(request.user_password.clone()).await {
        Ok(password_hash) => password_hash,
        Err(error) => {
            request.zeroize();
            return Err(ApiError::from_source(CodeError::PASSWORD_HASH_ERROR, error));
        }
    };

    let mut conn = match postgres_conn(&state).await {
        Ok(conn) => conn,
        Err(error) => {
            request.zeroize();
            return Err(error);
        }
    };

    let new_user = NewUser {
        user_name: request.user_name.trim().to_string(),
        user_email: user_email.clone(),
        user_password_hash: password_hash,
        user_country: request.user_country,
        user_language: request.user_language,
        user_subdivision: request.user_subdivision,
    };

    let user = match user_repository::insert_user(&mut conn, new_user).await {
        Ok(user) => user,
        Err(InsertUserError::UniqueViolation) => {
            request.zeroize();
            return Err(ApiError::new(CodeError::EMAIL_ALREADY_EXISTS));
        }
        Err(error) => {
            request.zeroize();
            return Err(ApiError::from_source(CodeError::DB_INSERT_ERROR, error));
        }
    };

    match user_role_repository::insert_for_user(&mut conn, user.user_id, RoleType::User).await {
        Ok(()) => {}
        Err(error) => {
            request.zeroize();
            return Err(ApiError::from_source(CodeError::DB_INSERT_ERROR, error));
        }
    }

    let now = Utc::now();
    let email_verification_token = Uuid::now_v7();
    let verify_by = now + EMAIL_VERIFICATION_TOKEN_VALID_DURATION;
    let new_token = NewEmailVerificationToken {
        user_id: user.user_id,
        email_verification_token,
        email_verification_token_expires_at: verify_by,
        email_verification_token_created_at: now,
    };

    match email_verification_token_repository::insert_token(&mut conn, new_token).await {
        Ok(_) => {}
        Err(error) => {
            request.zeroize();
            return Err(ApiError::from_source(CodeError::DB_INSERT_ERROR, error));
        }
    }

    drop(conn);
    request.zeroize();
    queue_verification_email(
        state,
        user.user_email.clone(),
        email_verification_token,
        verify_by,
    );

    Ok(SignupResponse {
        user_id: user.user_id,
        user_name: user.user_name,
        user_email: user.user_email,
        verify_by,
    })
}
