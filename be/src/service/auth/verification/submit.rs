use std::{sync::Arc, time::Duration as StdDuration};

use chrono::Utc;
use diesel_async::AsyncConnection;

use crate::{
    dto::{
        api_response::ApiResult,
        auth::{request::VerifyEmailChallengeRequest, response::VerifyEmailResponse},
    },
    error::{api_error::ApiError, code_error::CodeError},
    init::state::server_state::ServerState,
    repository::auth::postgres::{
        email_verification_challenge_repository::challenge, email_verification_token_repository,
        user_repository,
    },
    service::auth::{
        datasource::postgres_conn,
        verification::{
            consts::{CHALLENGE_SUBMIT_LIMIT_PER_CHALLENGE, RATE_WINDOW_SECONDS},
            validation::{record_failed_attempt, validate_challenge_submission},
        },
    },
};

pub async fn verify_user_email(
    state: Arc<ServerState>,
    request: VerifyEmailChallengeRequest,
) -> ApiResult<VerifyEmailResponse> {
    if !state
        .email_verification_challenge_cache
        .check_rate_limit(
            format!(
                "email-verification:submit:challenge:{}",
                request.email_verification_challenge_id
            ),
            CHALLENGE_SUBMIT_LIMIT_PER_CHALLENGE,
            StdDuration::from_secs(RATE_WINDOW_SECONDS),
        )
        .await
    {
        return Err(ApiError::new(CodeError::RATE_LIMITED));
    }

    let cached_challenge = state
        .email_verification_challenge_cache
        .challenge(request.email_verification_challenge_id)
        .await;
    let now = Utc::now();
    let mut conn = match postgres_conn(&state).await {
        Ok(conn) => conn,
        Err(error) => return Err(error),
    };

    let transaction_result = {
        let conn = &mut *conn;
        conn.transaction::<_, ApiError, _>(async |conn| {
            let verification_token = match email_verification_token_repository::find_by_token(
                &mut *conn,
                request.email_validation_token_id,
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

            let challenge = match challenge::find_challenge_by_id(
                &mut *conn,
                request.email_verification_challenge_id,
            )
            .await
            {
                Ok(Some(challenge)) => challenge,
                Ok(None) => {
                    return Err(ApiError::new(
                        CodeError::EMAIL_VERIFICATION_CHALLENGE_INVALID,
                    ));
                }
                Err(error) => {
                    return Err(ApiError::from_source(CodeError::DB_QUERY_ERROR, error));
                }
            };
            if challenge.email_verification_token_id
                != verification_token.email_verification_token_id
            {
                return Err(ApiError::new(
                    CodeError::EMAIL_VERIFICATION_CHALLENGE_INVALID,
                ));
            }

            match validate_challenge_submission(
                &state,
                &challenge,
                cached_challenge.as_ref(),
                &request,
                now,
            )
            .await
            {
                Ok(()) => {}
                Err(error) => {
                    record_failed_attempt(&mut *conn, &challenge, &error, now).await;
                    return Err(error);
                }
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
            match challenge::mark_challenge_solved(
                &mut *conn,
                challenge.email_verification_challenge_id,
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
    };
    let user = match transaction_result {
        Ok(user) => user,
        Err(error) => return Err(error),
    };

    state
        .email_verification_challenge_cache
        .remove_challenge(request.email_verification_challenge_id)
        .await;

    Ok(VerifyEmailResponse {
        user_id: user.user_id,
        user_email: user.user_email,
        verified_at: now,
    })
}
