use std::sync::Arc;

use chrono::{Duration, Utc};
use email_address::EmailAddress;
use tracing::error;
use uuid::Uuid;
use zeroize::Zeroize;

use crate::{
    domain::auth::user::NewPasswordResetToken,
    dto::{
        api_response::ApiResult,
        auth::{
            request::{ResetPasswordProcessRequest, ResetPasswordRequest},
            response::{ResetPasswordRequestResponse, ResetPasswordResponse},
        },
    },
    error::{api_error::ApiError, code_error::CodeError},
    init::state::server_state::ServerState,
    repository::auth::postgres::{password_reset_token_repository, user_repository},
    service::auth::{datasource::postgres_conn, email::queue_password_reset_email},
    util::{
        crypto::password::hash_password,
        string::validation::{normalized_email, validate_password_form},
    },
};

const PASSWORD_RESET_TOKEN_VALID_DURATION: Duration = Duration::minutes(30);

pub async fn request_password_reset(
    state: Arc<ServerState>,
    request: ResetPasswordRequest,
) -> ApiResult<ResetPasswordRequestResponse> {
    if !EmailAddress::is_valid(&request.user_email) {
        return Err(ApiError::new(CodeError::EMAIL_INVALID));
    }

    let mut conn = match postgres_conn(&state).await {
        Ok(conn) => conn,
        Err(error) => return Err(error),
    };

    let email = normalized_email(&request.user_email);
    let user = match user_repository::find_user_by_email(&mut conn, &email).await {
        Ok(Some(user)) => user,
        Ok(None) => return Err(ApiError::new(CodeError::USER_NOT_FOUND)),
        Err(error) => return Err(ApiError::from_source(CodeError::DB_QUERY_ERROR, error)),
    };

    let now = Utc::now();
    let password_reset_token = Uuid::now_v7();
    let verify_by = now + PASSWORD_RESET_TOKEN_VALID_DURATION;
    let new_token = NewPasswordResetToken {
        user_id: user.user_id,
        password_reset_token,
        password_reset_token_expires_at: verify_by,
        password_reset_token_created_at: now,
    };

    match password_reset_token_repository::insert_token(&mut conn, new_token).await {
        Ok(_) => {}
        Err(error) => return Err(ApiError::from_source(CodeError::DB_INSERT_ERROR, error)),
    }

    drop(conn);
    queue_password_reset_email(
        state,
        user.user_email.clone(),
        password_reset_token,
        verify_by,
    );

    Ok(ResetPasswordRequestResponse {
        user_email: user.user_email,
        verify_by,
        delivery_queued: true,
    })
}

pub async fn reset_password(
    state: Arc<ServerState>,
    mut request: ResetPasswordProcessRequest,
) -> ApiResult<ResetPasswordResponse> {
    if !validate_password_form(&request.new_password) {
        return Err(ApiError::new(CodeError::PASSWORD_INVALID));
    }

    let mut conn = match postgres_conn(&state).await {
        Ok(conn) => conn,
        Err(error) => {
            request.zeroize();
            return Err(error);
        }
    };

    let now = Utc::now();
    let reset_token = match password_reset_token_repository::find_by_token(
        &mut conn,
        request.password_reset_token,
    )
    .await
    {
        Ok(Some(reset_token)) => reset_token,
        Ok(None) => {
            request.zeroize();
            return Err(ApiError::new(CodeError::PASSWORD_RESET_TOKEN_INVALID));
        }
        Err(error) => {
            request.zeroize();
            return Err(ApiError::from_source(CodeError::DB_QUERY_ERROR, error));
        }
    };

    if reset_token.password_reset_token_used_at.is_some() {
        request.zeroize();
        return Err(ApiError::new(CodeError::PASSWORD_RESET_TOKEN_ALREADY_USED));
    }
    if reset_token.password_reset_token_created_at > now
        || reset_token.password_reset_token_expires_at < now
    {
        request.zeroize();
        return Err(ApiError::new(CodeError::PASSWORD_RESET_TOKEN_EXPIRED));
    }

    let new_password_hash = match hash_password(request.new_password.clone()).await {
        Ok(new_password_hash) => new_password_hash,
        Err(error) => {
            request.zeroize();
            return Err(ApiError::from_source(CodeError::PASSWORD_HASH_ERROR, error));
        }
    };
    request.zeroize();

    let user = match user_repository::update_password_after_reset(
        &mut conn,
        reset_token.user_id,
        new_password_hash,
        now,
    )
    .await
    {
        Ok(user) => user,
        Err(error) => return Err(ApiError::from_source(CodeError::DB_UPDATE_ERROR, error)),
    };

    match password_reset_token_repository::mark_used(
        &mut conn,
        reset_token.password_reset_token_id,
        now,
    )
    .await
    {
        Ok(_) => {}
        Err(error) => {
            error!(
                error = %error,
                password_reset_token_id = %reset_token.password_reset_token_id,
                "Failed to mark password reset token as used"
            );
        }
    }

    Ok(ResetPasswordResponse {
        user_id: user.user_id,
        user_name: user.user_name,
        user_email: user.user_email,
        user_updated_at: user.user_updated_at,
    })
}
