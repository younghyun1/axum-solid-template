use std::sync::Arc;

use axum::{
    Extension, Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::{
    dto::{
        api_response::{ApiEnvelope, ApiResponseResult},
        auth::{
            request::{
                CreateEmailVerificationQuestionAnswerRequest,
                CreateEmailVerificationQuestionRequest,
            },
            response::{DatabaseResetResponse, EmailVerificationQuestionnaireResponse},
        },
    },
    init::state::server_state::ServerState,
    middleware::auth::AuthContext,
    service::{
        admin::reset_database as reset_database_service,
        auth::verification::admin::{
            add_email_verification_question_answer, create_email_verification_question,
            delete_email_verification_question, delete_email_verification_question_answer,
            list_email_verification_questions,
        },
    },
};

use super::support::response_from_result;

#[utoipa::path(
    get,
    path = "/api/v1/admin/email-verification/questions",
    tag = "admin",
    responses((status = 200, description = "Email verification questionnaire", body = ApiEnvelope<EmailVerificationQuestionnaireResponse>))
)]
pub async fn admin_email_verification_questions(
    State(state): State<Arc<ServerState>>,
) -> ApiResponseResult<EmailVerificationQuestionnaireResponse> {
    response_from_result(list_email_verification_questions(state).await)
}

#[utoipa::path(
    post,
    path = "/api/v1/admin/email-verification/questions",
    tag = "admin",
    request_body = CreateEmailVerificationQuestionRequest,
    responses((status = 200, description = "Email verification questionnaire", body = ApiEnvelope<EmailVerificationQuestionnaireResponse>))
)]
pub async fn admin_create_email_verification_question(
    Extension(auth_context): Extension<AuthContext>,
    State(state): State<Arc<ServerState>>,
    Json(request): Json<CreateEmailVerificationQuestionRequest>,
) -> ApiResponseResult<EmailVerificationQuestionnaireResponse> {
    response_from_result(create_email_verification_question(state, auth_context, request).await)
}

#[utoipa::path(
    delete,
    path = "/api/v1/admin/email-verification/questions/{question_id}",
    tag = "admin",
    params(("question_id" = Uuid, Path, description = "Question id")),
    responses((status = 200, description = "Email verification questionnaire", body = ApiEnvelope<EmailVerificationQuestionnaireResponse>))
)]
pub async fn admin_delete_email_verification_question(
    Extension(auth_context): Extension<AuthContext>,
    State(state): State<Arc<ServerState>>,
    Path(question_id): Path<Uuid>,
) -> ApiResponseResult<EmailVerificationQuestionnaireResponse> {
    response_from_result(delete_email_verification_question(state, auth_context, question_id).await)
}

#[utoipa::path(
    post,
    path = "/api/v1/admin/email-verification/questions/{question_id}/answers",
    tag = "admin",
    params(("question_id" = Uuid, Path, description = "Question id")),
    request_body = CreateEmailVerificationQuestionAnswerRequest,
    responses((status = 200, description = "Email verification questionnaire", body = ApiEnvelope<EmailVerificationQuestionnaireResponse>))
)]
pub async fn admin_add_email_verification_question_answer(
    Extension(auth_context): Extension<AuthContext>,
    State(state): State<Arc<ServerState>>,
    Path(question_id): Path<Uuid>,
    Json(request): Json<CreateEmailVerificationQuestionAnswerRequest>,
) -> ApiResponseResult<EmailVerificationQuestionnaireResponse> {
    response_from_result(
        add_email_verification_question_answer(state, auth_context, question_id, request).await,
    )
}

#[utoipa::path(
    delete,
    path = "/api/v1/admin/email-verification/questions/{question_id}/answers/{answer_id}",
    tag = "admin",
    params(
        ("question_id" = Uuid, Path, description = "Question id"),
        ("answer_id" = Uuid, Path, description = "Answer id")
    ),
    responses((status = 200, description = "Email verification questionnaire", body = ApiEnvelope<EmailVerificationQuestionnaireResponse>))
)]
pub async fn admin_delete_email_verification_question_answer(
    Extension(auth_context): Extension<AuthContext>,
    State(state): State<Arc<ServerState>>,
    Path((question_id, answer_id)): Path<(Uuid, Uuid)>,
) -> ApiResponseResult<EmailVerificationQuestionnaireResponse> {
    response_from_result(
        delete_email_verification_question_answer(state, auth_context, question_id, answer_id)
            .await,
    )
}

#[utoipa::path(
    post,
    path = "/api/v1/admin/database/reset",
    tag = "admin",
    responses((status = 200, description = "Database reset", body = ApiEnvelope<DatabaseResetResponse>))
)]
pub async fn admin_reset_database(
    State(state): State<Arc<ServerState>>,
) -> ApiResponseResult<DatabaseResetResponse> {
    response_from_result(reset_database_service(state).await)
}
