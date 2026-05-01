use std::sync::Arc;

use chrono::Utc;
use diesel_async::AsyncConnection;

use crate::{
    dto::{
        api_response::ApiResult,
        auth::{request::EmailValidationToken, response::VerifyEmailResponse},
    },
    error::{api_error::ApiError, code_error::CodeError},
    init::state::server_state::ServerState,
    repository::auth::postgres::{email_verification_token_repository, user_repository},
    service::auth::datasource::postgres_conn,
};

pub async fn verify_user_email(
    state: Arc<ServerState>,
    token: EmailValidationToken,
) -> ApiResult<VerifyEmailResponse> {
    let mut conn = match postgres_conn(&state).await {
        Ok(conn) => conn,
        Err(error) => return Err(error),
    };

    let now = Utc::now();
    let email_validation_token_id = token.email_validation_token_id;
    let user = match {
        let conn = &mut *conn;

        conn.transaction::<_, ApiError, _>(async |conn| {
            let verification_token = match email_verification_token_repository::find_by_token(
                &mut *conn,
                email_validation_token_id,
            )
            .await
            {
                Ok(Some(verification_token)) => verification_token,
                Ok(None) => {
                    return Err(ApiError::new(CodeError::EMAIL_VERIFICATION_TOKEN_INVALID));
                }
                Err(error) => {
                    return Err(ApiError::from_source(CodeError::DB_QUERY_ERROR, error));
                }
            };

            if verification_token
                .email_verification_token_used_at
                .is_some()
            {
                return Err(ApiError::new(
                    CodeError::EMAIL_VERIFICATION_TOKEN_ALREADY_USED,
                ));
            }
            if verification_token.email_verification_token_created_at > now
                || verification_token.email_verification_token_expires_at < now
            {
                return Err(ApiError::new(CodeError::EMAIL_VERIFICATION_TOKEN_EXPIRED));
            }

                let user = match user_repository::mark_email_verified(
                &mut *conn,
                verification_token.user_id,
                now,
            )
            .await
            {
                Ok(user) => user,
                Err(error) => {
                    return Err(ApiError::from_source(CodeError::DB_UPDATE_ERROR, error));
                }
            };

                match email_verification_token_repository::mark_used(
                &mut *conn,
                verification_token.email_verification_token_id,
                now,
            )
            .await
            {
                Ok(_) => {}
                Err(error) => {
                    return Err(ApiError::from_source(CodeError::DB_UPDATE_ERROR, error));
                }
            }

            Ok(user)
        })
        .await
    } {
        Ok(user) => user,
        Err(error) => return Err(error),
    };

    Ok(VerifyEmailResponse {
        user_id: user.user_id,
        user_email: user.user_email,
        verified_at: now,
    })
}
