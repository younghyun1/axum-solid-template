use chrono::{DateTime, Utc};
use tracing::{info, warn};

use crate::{
    domain::auth::email_verification_challenge::{
        CHALLENGE_STATUS_ISSUED, EmailVerificationChallengeRow, POW_ALGORITHM_SHA256_V1,
    },
    dto::{api_response::ApiResult, auth::request::VerifyEmailChallengeRequest},
    error::{api_error::ApiError, code_error::CodeError},
    init::state::server_state::ServerState,
    repository::auth::postgres::email_verification_challenge_repository::challenge,
    service::auth::verification::{
        answer::{find_question, normalize_answer},
        consts::MAX_FAILED_ATTEMPTS,
        pow::verify_pow,
    },
};

pub async fn validate_challenge_submission(
    state: &ServerState,
    challenge: &EmailVerificationChallengeRow,
    cached_challenge: Option<&EmailVerificationChallengeRow>,
    request: &VerifyEmailChallengeRequest,
    now: DateTime<Utc>,
) -> ApiResult<()> {
    if challenge.email_verification_challenge_status != CHALLENGE_STATUS_ISSUED {
        return Err(ApiError::new(
            CodeError::EMAIL_VERIFICATION_CHALLENGE_INVALID,
        ));
    }
    if challenge.email_verification_challenge_expires_at < now {
        return Err(ApiError::new(
            CodeError::EMAIL_VERIFICATION_CHALLENGE_EXPIRED,
        ));
    }
    if !request.email_verification_honeypot.trim().is_empty() {
        return Err(ApiError::new(
            CodeError::EMAIL_VERIFICATION_CHALLENGE_FAILED,
        ));
    }

    let elapsed_ms = now
        .signed_duration_since(challenge.email_verification_challenge_issued_at)
        .num_milliseconds();
    if elapsed_ms < i64::from(challenge.email_verification_challenge_minimum_elapsed_ms) {
        return Err(ApiError::new(
            CodeError::EMAIL_VERIFICATION_CHALLENGE_FAILED,
        ));
    }

    let pow_challenge = match cached_challenge {
        Some(cached_challenge) => cached_challenge,
        None => challenge,
    };
    if pow_challenge.email_verification_challenge_pow_algorithm != POW_ALGORITHM_SHA256_V1 {
        return Err(ApiError::new(
            CodeError::EMAIL_VERIFICATION_CHALLENGE_INVALID,
        ));
    }
    if !verify_pow(
        pow_challenge.email_verification_challenge_id,
        &pow_challenge.email_verification_challenge_pow_salt,
        &request.email_verification_pow_nonce,
        pow_challenge.email_verification_challenge_pow_difficulty_bits,
    ) {
        return Err(ApiError::new(
            CodeError::EMAIL_VERIFICATION_CHALLENGE_FAILED,
        ));
    }

    let normalized_answer = normalize_answer(&request.email_verification_answer);
    let snapshot = state
        .email_verification_challenge_cache
        .questionnaire_snapshot()
        .await;
    let question = match find_question(&snapshot, challenge.email_verification_question_id) {
        Some(question) => question,
        None => {
            let refreshed = match state
                .email_verification_challenge_cache
                .refresh_questionnaire(&state.db_pool)
                .await
            {
                Ok(refreshed) => refreshed,
                Err(error) => return Err(ApiError::from_source(CodeError::DB_QUERY_ERROR, error)),
            };
            match find_question(&refreshed, challenge.email_verification_question_id) {
                Some(question) => question,
                None => {
                    return Err(ApiError::new(
                        CodeError::EMAIL_VERIFICATION_CHALLENGE_INVALID,
                    ));
                }
            }
        }
    };

    for answer in &question.email_verification_question_answers {
        if normalized_answer == answer.email_verification_question_answer_normalized {
            info!(
                challenge_id = %challenge.email_verification_challenge_id,
                question_id = %question.email_verification_question_id,
                elapsed_ms,
                "Email verification challenge solved"
            );
            return Ok(());
        }
    }

    Err(ApiError::new(CodeError::EMAIL_VERIFICATION_ANSWER_INVALID))
}

pub async fn record_failed_attempt(
    conn: &mut diesel_async::AsyncPgConnection,
    challenge: &EmailVerificationChallengeRow,
    error: &ApiError,
    now: DateTime<Utc>,
) {
    match challenge::record_challenge_attempt(
        conn,
        challenge.email_verification_challenge_id,
        &error.to_string(),
    )
    .await
    {
        Ok(_) => {}
        Err(update_error) => {
            warn!(
                error = %update_error,
                challenge_id = %challenge.email_verification_challenge_id,
                "Failed to record email verification challenge attempt"
            );
        }
    }
    let failed_attempts = challenge.email_verification_challenge_attempt_count + 1;
    if failed_attempts < MAX_FAILED_ATTEMPTS {
        return;
    }
    match challenge::mark_challenge_failed(conn, challenge.email_verification_challenge_id, now)
        .await
    {
        Ok(_) => {}
        Err(update_error) => {
            warn!(
                error = %update_error,
                challenge_id = %challenge.email_verification_challenge_id,
                "Failed to mark email verification challenge failed"
            );
        }
    }
}
