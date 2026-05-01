use std::sync::Arc;

use chrono::{Duration, Utc};
use diesel_async::AsyncConnection;
use uuid::Uuid;
use zeroize::Zeroize;

use crate::{
    domain::auth::{
        role::RoleType,
        user::NewEmailVerificationToken,
        user::NewUser,
        value::{IsoNumericCode, UserEmail, UserName},
    },
    dto::{
        api_response::ApiResult,
        auth::{
            request::{SignupRequest, SignupRole},
            response::SignupResponse,
        },
    },
    error::{api_error::ApiError, code_error::CodeError},
    init::state::server_state::ServerState,
    repository::auth::postgres::{
        email_verification_token_repository,
        user_repository::{self, InsertUserError},
        user_role_repository,
    },
    service::auth::{datasource::postgres_conn, email::queue_verification_email},
    util::{crypto::password::hash_password, string::validation::validate_password_form},
};

const EMAIL_VERIFICATION_TOKEN_VALID_DURATION: Duration = Duration::days(1);

pub async fn signup_user(
    state: Arc<ServerState>,
    mut request: SignupRequest,
) -> ApiResult<SignupResponse> {
    let user_name = match UserName::try_new(request.user_name.clone()) {
        Ok(user_name) => user_name,
        Err(_) => return Err(ApiError::new(CodeError::USER_NAME_INVALID)),
    };
    let user_email = match UserEmail::try_new(request.user_email.clone()) {
        Ok(user_email) => user_email,
        Err(_) => return Err(ApiError::new(CodeError::EMAIL_INVALID)),
    };
    match IsoNumericCode::try_new(request.user_country) {
        Ok(_) => {}
        Err(_) => return Err(ApiError::new(CodeError::INTERNAL_ERROR)),
    }
    match IsoNumericCode::try_new(request.user_language) {
        Ok(_) => {}
        Err(_) => return Err(ApiError::new(CodeError::INTERNAL_ERROR)),
    }
    if let Some(user_subdivision) = request.user_subdivision {
        match IsoNumericCode::try_new(user_subdivision) {
            Ok(_) => {}
            Err(_) => return Err(ApiError::new(CodeError::INTERNAL_ERROR)),
        }
    }
    if !validate_password_form(&request.user_password) {
        return Err(ApiError::new(CodeError::PASSWORD_INVALID));
    }

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

    let now = Utc::now();
    let email_verification_token = Uuid::now_v7();
    let verify_by = now + EMAIL_VERIFICATION_TOKEN_VALID_DURATION;
    let user_country = request.user_country;
    let user_language = request.user_language;
    let user_subdivision = request.user_subdivision;
    let user_role = match request.user_role {
        SignupRole::User => RoleType::User,
        SignupRole::ServiceProvider => RoleType::ServiceProvider,
    };
    let signup_result = {
        let conn = &mut *conn;

        conn.transaction::<_, ApiError, _>(async |conn| {
            let new_user = NewUser {
                user_name: user_name.into_inner(),
                user_email: user_email.into_inner(),
                user_password_hash: password_hash,
                user_country,
                user_language,
                user_subdivision,
            };

                let user = match user_repository::insert_user(&mut *conn, new_user).await {
                Ok(user) => user,
                Err(InsertUserError::UniqueViolation) => {
                    return Err(ApiError::new(CodeError::EMAIL_ALREADY_EXISTS));
                }
                Err(error) => {
                    return Err(ApiError::from_source(CodeError::DB_INSERT_ERROR, error));
                }
            };

                match user_role_repository::insert_for_user(&mut *conn, user.user_id, user_role).await {
                Ok(()) => {}
                Err(error) => {
                    return Err(ApiError::from_source(CodeError::DB_INSERT_ERROR, error));
                }
            }

            let new_token = NewEmailVerificationToken {
                user_id: user.user_id,
                email_verification_token,
                email_verification_token_expires_at: verify_by,
                email_verification_token_created_at: now,
            };

                match email_verification_token_repository::insert_token(&mut *conn, new_token).await {
                Ok(_) => {}
                Err(error) => {
                    return Err(ApiError::from_source(CodeError::DB_INSERT_ERROR, error));
                }
            }

            Ok(user)
        })
        .await
    };

    request.zeroize();
    let user = match signup_result {
        Ok(user) => user,
        Err(error) => return Err(error),
    };

    drop(conn);
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
