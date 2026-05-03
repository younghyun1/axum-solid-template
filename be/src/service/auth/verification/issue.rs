use std::{net::SocketAddr, sync::Arc, time::Duration as StdDuration};

use chrono::{Duration, Utc};
use diesel_async::AsyncConnection;
use uuid::Uuid;

use crate::{
    domain::auth::email_verification_challenge::{
        EmailVerificationQuestion, NewEmailVerificationChallenge, POW_ALGORITHM_SHA256_V1,
    },
    dto::{
        api_response::ApiResult,
        auth::{request::EmailValidationToken, response::EmailVerificationChallengeResponse},
    },
    error::{api_error::ApiError, code_error::CodeError},
    init::state::server_state::ServerState,
    repository::auth::postgres::{
        email_verification_challenge_repository::challenge, email_verification_token_repository,
    },
    service::auth::{
        datasource::postgres_conn,
        verification::{
            consts::{
                CHALLENGE_ISSUE_LIMIT_PER_IP, CHALLENGE_ISSUE_LIMIT_PER_TOKEN,
                CHALLENGE_VALID_MINUTES, MINIMUM_ELAPSED_MS, POW_DIFFICULTY_BITS,
                RATE_WINDOW_SECONDS,
            },
            response::challenge_response,
        },
    },
};

pub async fn issue_email_verification_challenge(
    state: Arc<ServerState>,
    token: EmailValidationToken,
    client_addr: Option<SocketAddr>,
    user_agent: Option<String>,
) -> ApiResult<EmailVerificationChallengeResponse> {
    let ip_key = client_ip_key(client_addr);
    if !state
        .email_verification_challenge_cache
        .check_rate_limit(
            format!("email-verification:issue:ip:{ip_key}"),
            CHALLENGE_ISSUE_LIMIT_PER_IP,
            StdDuration::from_secs(RATE_WINDOW_SECONDS),
        )
        .await
    {
        return Err(ApiError::new(CodeError::RATE_LIMITED));
    }
    if !state
        .email_verification_challenge_cache
        .check_rate_limit(
            format!(
                "email-verification:issue:token:{}",
                token.email_validation_token_id
            ),
            CHALLENGE_ISSUE_LIMIT_PER_TOKEN,
            StdDuration::from_secs(RATE_WINDOW_SECONDS),
        )
        .await
    {
        return Err(ApiError::new(CodeError::RATE_LIMITED));
    }

    let mut questionnaire = state
        .email_verification_challenge_cache
        .questionnaire_snapshot()
        .await;
    let mut public_questions = questionnaire.public_questions();
    if public_questions.is_empty() {
        questionnaire = match state
            .email_verification_challenge_cache
            .refresh_questionnaire(&state.db_pool)
            .await
        {
            Ok(questionnaire) => questionnaire,
            Err(error) => return Err(ApiError::from_source(CodeError::DB_QUERY_ERROR, error)),
        };
        public_questions = questionnaire.public_questions();
    }
    if public_questions.is_empty() {
        return Err(ApiError::public(
            CodeError::VALIDATION_FAILED,
            "No active email verification questions are configured.",
        ));
    }

    let selected_question = match select_question(&public_questions) {
        Some(selected_question) => selected_question,
        None => {
            return Err(ApiError::public(
                CodeError::VALIDATION_FAILED,
                "No active email verification questions are configured.",
            ));
        }
    };
    let now = Utc::now();
    let expires_at = now + Duration::minutes(CHALLENGE_VALID_MINUTES);
    let challenge_id = Uuid::now_v7();
    let challenge = NewEmailVerificationChallenge {
        email_verification_challenge_id: challenge_id,
        email_verification_token_id: Uuid::nil(),
        email_verification_question_id: selected_question.email_verification_question_id,
        email_verification_challenge_pow_salt: Uuid::now_v7().to_string(),
        email_verification_challenge_pow_difficulty_bits: POW_DIFFICULTY_BITS,
        email_verification_challenge_pow_algorithm: POW_ALGORITHM_SHA256_V1.to_string(),
        email_verification_challenge_minimum_elapsed_ms: MINIMUM_ELAPSED_MS,
        email_verification_challenge_issued_at: now,
        email_verification_challenge_expires_at: expires_at,
        email_verification_challenge_client_ip: client_addr.map(|addr| addr.ip().to_string()),
        email_verification_challenge_user_agent: user_agent,
    };

    let mut conn = match postgres_conn(&state).await {
        Ok(conn) => conn,
        Err(error) => return Err(error),
    };
    let transaction_result = {
        let conn = &mut *conn;
        conn.transaction::<_, ApiError, _>(async |conn| {
            let verification_token = match email_verification_token_repository::find_by_token(
                &mut *conn,
                token.email_validation_token_id,
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

            match challenge::supersede_issued_challenges(
                &mut *conn,
                verification_token.email_verification_token_id,
            )
            .await
            {
                Ok(_) => {}
                Err(error) => {
                    return Err(ApiError::from_source(CodeError::DB_UPDATE_ERROR, error));
                }
            }

            let challenge = NewEmailVerificationChallenge {
                email_verification_token_id: verification_token.email_verification_token_id,
                ..challenge
            };
            match challenge::insert_challenge(&mut *conn, challenge).await {
                Ok(challenge) => Ok(challenge),
                Err(error) => Err(ApiError::from_source(CodeError::DB_INSERT_ERROR, error)),
            }
        })
        .await
    };
    let challenge_row = match transaction_result {
        Ok(challenge) => challenge,
        Err(error) => return Err(error),
    };

    state
        .email_verification_challenge_cache
        .store_challenge(challenge_row.clone())
        .await;

    Ok(challenge_response(
        &challenge_row,
        &selected_question,
        questionnaire.email_verification_questionnaire_revision,
    ))
}

fn select_question(questions: &[EmailVerificationQuestion]) -> Option<EmailVerificationQuestion> {
    if questions.is_empty() {
        return None;
    }
    let index = (Uuid::now_v7().as_u128() as usize) % questions.len();
    match questions.get(index) {
        Some(question) => Some(question.clone()),
        None => questions.first().cloned(),
    }
}

fn client_ip_key(client_addr: Option<SocketAddr>) -> String {
    match client_addr {
        Some(client_addr) => client_addr.ip().to_string(),
        None => "unknown".to_string(),
    }
}
