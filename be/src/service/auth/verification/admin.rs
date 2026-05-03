use std::sync::Arc;

use chrono::Utc;
use diesel_async::{AsyncConnection, AsyncPgConnection};
use uuid::Uuid;

use crate::{
    domain::auth::email_verification_challenge::EmailVerificationQuestionnaireSnapshot,
    dto::{
        api_response::ApiResult,
        auth::{
            request::{
                CreateEmailVerificationQuestionAnswerRequest,
                CreateEmailVerificationQuestionRequest,
            },
            response::EmailVerificationQuestionnaireResponse,
        },
    },
    error::{api_error::ApiError, code_error::CodeError},
    init::state::server_state::ServerState,
    middleware::auth::AuthContext,
    repository::auth::postgres::email_verification_challenge_repository::{
        admin_mutation, questionnaire,
    },
    service::auth::{
        datasource::postgres_conn,
        verification::{
            answer::{normalize_answer, normalize_answer_set},
            response::questionnaire_response,
        },
    },
};

pub async fn list_email_verification_questions(
    state: Arc<ServerState>,
) -> ApiResult<EmailVerificationQuestionnaireResponse> {
    let snapshot = state
        .email_verification_challenge_cache
        .questionnaire_snapshot()
        .await;
    Ok(questionnaire_response(snapshot))
}

pub async fn create_email_verification_question(
    state: Arc<ServerState>,
    auth_context: AuthContext,
    request: CreateEmailVerificationQuestionRequest,
) -> ApiResult<EmailVerificationQuestionnaireResponse> {
    let prompt = request.email_verification_question_prompt.trim();
    if prompt.len() < 8 {
        return Err(ApiError::public(
            CodeError::VALIDATION_FAILED,
            "Question prompt must be at least 8 characters.",
        ));
    }

    let answers = normalize_answer_set(request.email_verification_question_answers);
    if answers.is_empty() {
        return Err(ApiError::public(
            CodeError::VALIDATION_FAILED,
            "At least one answer is required.",
        ));
    }

    let mut conn = match postgres_conn(&state).await {
        Ok(conn) => conn,
        Err(error) => return Err(error),
    };
    let admin_user_id = auth_context.claims.user_id;
    let transaction_result = {
        let conn = &mut *conn;
        conn.transaction::<_, ApiError, _>(async |conn| {
            let question_id =
                match admin_mutation::insert_question(&mut *conn, prompt, admin_user_id).await {
                    Ok(question_id) => question_id,
                    Err(error) => {
                        return Err(ApiError::from_source(CodeError::DB_INSERT_ERROR, error));
                    }
                };
            for answer in &answers {
                match admin_mutation::insert_answer(
                    &mut *conn,
                    question_id,
                    &answer.answer_text,
                    &answer.answer_normalized,
                    admin_user_id,
                )
                .await
                {
                    Ok(_) => {}
                    Err(error) => {
                        return Err(ApiError::from_source(CodeError::DB_INSERT_ERROR, error));
                    }
                }
            }
            match questionnaire::bump_questionnaire_revision(&mut *conn).await {
                Ok(_) => {}
                Err(error) => {
                    return Err(ApiError::from_source(CodeError::DB_UPDATE_ERROR, error));
                }
            }
            load_questionnaire_snapshot(&mut *conn).await
        })
        .await
    };
    drop(conn);
    synchronized_questionnaire_response(state, transaction_result).await
}

pub async fn add_email_verification_question_answer(
    state: Arc<ServerState>,
    auth_context: AuthContext,
    question_id: Uuid,
    request: CreateEmailVerificationQuestionAnswerRequest,
) -> ApiResult<EmailVerificationQuestionnaireResponse> {
    let answer_text = request.email_verification_question_answer_text.trim();
    let answer_normalized = normalize_answer(answer_text);
    if answer_normalized.is_empty() {
        return Err(ApiError::public(
            CodeError::VALIDATION_FAILED,
            "Answer must not be empty.",
        ));
    }

    let mut conn = match postgres_conn(&state).await {
        Ok(conn) => conn,
        Err(error) => return Err(error),
    };
    let transaction_result = {
        let conn = &mut *conn;
        conn.transaction::<_, ApiError, _>(async |conn| {
            match admin_mutation::insert_answer(
                &mut *conn,
                question_id,
                answer_text,
                &answer_normalized,
                auth_context.claims.user_id,
            )
            .await
            {
                Ok(_) => {}
                Err(error) => {
                    return Err(ApiError::from_source(CodeError::DB_INSERT_ERROR, error));
                }
            }
            match questionnaire::bump_questionnaire_revision(&mut *conn).await {
                Ok(_) => {}
                Err(error) => {
                    return Err(ApiError::from_source(CodeError::DB_UPDATE_ERROR, error));
                }
            }
            load_questionnaire_snapshot(&mut *conn).await
        })
        .await
    };
    drop(conn);
    synchronized_questionnaire_response(state, transaction_result).await
}

pub async fn delete_email_verification_question(
    state: Arc<ServerState>,
    auth_context: AuthContext,
    question_id: Uuid,
) -> ApiResult<EmailVerificationQuestionnaireResponse> {
    let now = Utc::now();
    let mut conn = match postgres_conn(&state).await {
        Ok(conn) => conn,
        Err(error) => return Err(error),
    };
    let transaction_result = {
        let conn = &mut *conn;
        conn.transaction::<_, ApiError, _>(async |conn| {
            match admin_mutation::delete_question_answers(
                &mut *conn,
                question_id,
                auth_context.claims.user_id,
                now,
            )
            .await
            {
                Ok(_) => {}
                Err(error) => {
                    return Err(ApiError::from_source(CodeError::DB_UPDATE_ERROR, error));
                }
            }
            match admin_mutation::delete_question(
                &mut *conn,
                question_id,
                auth_context.claims.user_id,
                now,
            )
            .await
            {
                Ok(rows) => {
                    if rows == 0 {
                        return Err(ApiError::new(CodeError::REFERENCE_DATA_NOT_FOUND));
                    }
                }
                Err(error) => {
                    return Err(ApiError::from_source(CodeError::DB_UPDATE_ERROR, error));
                }
            }
            match questionnaire::bump_questionnaire_revision(&mut *conn).await {
                Ok(_) => {}
                Err(error) => {
                    return Err(ApiError::from_source(CodeError::DB_UPDATE_ERROR, error));
                }
            }
            load_questionnaire_snapshot(&mut *conn).await
        })
        .await
    };
    drop(conn);
    synchronized_questionnaire_response(state, transaction_result).await
}

pub async fn delete_email_verification_question_answer(
    state: Arc<ServerState>,
    auth_context: AuthContext,
    question_id: Uuid,
    answer_id: Uuid,
) -> ApiResult<EmailVerificationQuestionnaireResponse> {
    let now = Utc::now();
    let mut conn = match postgres_conn(&state).await {
        Ok(conn) => conn,
        Err(error) => return Err(error),
    };
    let transaction_result = {
        let conn = &mut *conn;
        conn.transaction::<_, ApiError, _>(async |conn| {
            match admin_mutation::delete_answer(
                &mut *conn,
                question_id,
                answer_id,
                auth_context.claims.user_id,
                now,
            )
            .await
            {
                Ok(rows) => {
                    if rows == 0 {
                        return Err(ApiError::new(CodeError::REFERENCE_DATA_NOT_FOUND));
                    }
                }
                Err(error) => {
                    return Err(ApiError::from_source(CodeError::DB_UPDATE_ERROR, error));
                }
            }
            match questionnaire::bump_questionnaire_revision(&mut *conn).await {
                Ok(_) => {}
                Err(error) => {
                    return Err(ApiError::from_source(CodeError::DB_UPDATE_ERROR, error));
                }
            }
            load_questionnaire_snapshot(&mut *conn).await
        })
        .await
    };
    drop(conn);
    synchronized_questionnaire_response(state, transaction_result).await
}

async fn load_questionnaire_snapshot(
    conn: &mut AsyncPgConnection,
) -> Result<EmailVerificationQuestionnaireSnapshot, ApiError> {
    match questionnaire::load_questionnaire_snapshot(conn).await {
        Ok(snapshot) => Ok(snapshot),
        Err(error) => Err(ApiError::from_source(CodeError::DB_QUERY_ERROR, error)),
    }
}

async fn synchronized_questionnaire_response(
    state: Arc<ServerState>,
    transaction_result: Result<EmailVerificationQuestionnaireSnapshot, ApiError>,
) -> ApiResult<EmailVerificationQuestionnaireResponse> {
    let snapshot = match transaction_result {
        Ok(snapshot) => snapshot,
        Err(error) => return Err(error),
    };
    state
        .email_verification_challenge_cache
        .replace_questionnaire(snapshot.clone())
        .await;
    Ok(questionnaire_response(snapshot))
}
