use std::sync::Arc;

use chrono::Utc;
use tracing::error;
use zeroize::Zeroize;

use crate::{
    domain::auth::value::UserEmail,
    dto::{
        api_response::ApiResult,
        auth::{request::LoginRequest, response::LoginResponse},
    },
    error::{api_error::ApiError, code_error::CodeError},
    init::state::server_state::ServerState,
    repository::auth::postgres::{user_repository, user_role_repository},
    service::auth::datasource::postgres_conn,
    util::{
        auth::jwt::{JWT_BEARER_TOKEN_TYPE, JwtUserContext, issue_access_token},
        crypto::password::verify_password,
        string::validation::validate_password_form,
    },
};

pub async fn login_user(
    state: Arc<ServerState>,
    mut request: LoginRequest,
) -> ApiResult<LoginResponse> {
    let user_email = match UserEmail::try_new(request.user_email.clone()) {
        Ok(user_email) => user_email,
        Err(_) => return Err(ApiError::new(CodeError::EMAIL_INVALID)),
    };
    if !validate_password_form(&request.user_password) {
        return Err(ApiError::new(CodeError::PASSWORD_INVALID));
    }

    let mut conn = match postgres_conn(&state).await {
        Ok(conn) => conn,
        Err(error) => {
            request.zeroize();
            return Err(error);
        }
    };

    let user_email = user_email.into_inner();
    let user = match user_repository::find_user_by_email(&mut conn, &user_email).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            request.zeroize();
            return Err(ApiError::new(CodeError::USER_NOT_FOUND));
        }
        Err(error) => {
            request.zeroize();
            return Err(ApiError::from_source(CodeError::DB_QUERY_ERROR, error));
        }
    };

    let password_matches = match verify_password(
        request.user_password.clone(),
        user.user_password_hash.clone(),
    )
    .await
    {
        Ok(password_matches) => password_matches,
        Err(error) => {
            request.zeroize();
            return Err(ApiError::from_source(
                CodeError::PASSWORD_VERIFY_ERROR,
                error,
            ));
        }
    };

    request.zeroize();
    if !password_matches {
        return Err(ApiError::new(CodeError::WRONG_PASSWORD));
    }
    if !user.user_is_email_verified {
        return Err(ApiError::new(CodeError::EMAIL_NOT_VERIFIED));
    }

    let role_type = match user_role_repository::role_for_user(&mut conn, user.user_id).await {
        Ok(role_type) => role_type,
        Err(error) => return Err(ApiError::from_source(CodeError::DB_QUERY_ERROR, error)),
    };

    match user_repository::update_last_login(&mut conn, user.user_id, Utc::now()).await {
        Ok(_) => {}
        Err(error) => {
            error!(error = %error, user_id = %user.user_id, "Failed to update user last login");
        }
    }

    let issued = match issue_access_token(
        &state.server_config.jwt_config,
        JwtUserContext { user, role_type },
    ) {
        Ok(issued) => issued,
        Err(error) => return Err(ApiError::from_source(CodeError::JWT_INVALID, error)),
    };

    Ok(LoginResponse {
        access_token: issued.token,
        token_type: JWT_BEARER_TOKEN_TYPE,
        expires_at: issued.expires_at,
        claims: issued.claims,
    })
}
