use std::{net::SocketAddr, sync::Arc};

use axum::{
    Extension, Json,
    extract::{ConnectInfo, Path, Query, State},
    http::HeaderMap,
};
use uuid::Uuid;

use crate::{
    dto::{
        api_response::{ApiEnvelope, ApiResponse, ApiResponseResult, ApiResult},
        auth::{
            request::{
                CheckIfUserExistsRequest, CreateEmailVerificationQuestionAnswerRequest,
                CreateEmailVerificationQuestionRequest, EmailValidationToken, LoginRequest,
                ResetPasswordProcessRequest, ResetPasswordRequest, SignupRequest,
                VerifyEmailChallengeRequest,
            },
            response::{
                CheckIfUserExistsResponse, EmailVerificationChallengeResponse,
                EmailVerificationQuestionnaireResponse, LoginResponse, LogoutResponse, MeResponse,
                PublicUserInfoResponse, ResetPasswordRequestResponse, ResetPasswordResponse,
                SignupResponse, VerifyEmailResponse,
            },
        },
    },
    error::prelude::*,
    init::state::server_state::ServerState,
    middleware::auth::AuthContext,
    service::auth::{
        login::login_user,
        password_reset::{request_password_reset, reset_password as reset_password_service},
        signup::signup_user,
        user::{
            check_if_user_exists as check_if_user_exists_service, current_user, public_user_info,
        },
        verification::{
            admin::{
                add_email_verification_question_answer, create_email_verification_question,
                delete_email_verification_question, delete_email_verification_question_answer,
                list_email_verification_questions,
            },
            issue::issue_email_verification_challenge,
            submit::verify_user_email as verify_user_email_service,
        },
    },
};

#[utoipa::path(
    post,
    path = "/api/v1/auth/signup",
    tag = "auth",
    request_body = SignupRequest,
    responses((status = 200, description = "User successfully signed up", body = ApiEnvelope<SignupResponse>))
)]
pub async fn signup(
    State(state): State<Arc<ServerState>>,
    Json(request): Json<SignupRequest>,
) -> ApiResponseResult<SignupResponse> {
    response_from_result(signup_user(state, request).await)
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    tag = "auth",
    request_body = LoginRequest,
    responses((status = 200, description = "Login successful", body = ApiEnvelope<LoginResponse>))
)]
pub async fn login(
    State(state): State<Arc<ServerState>>,
    Json(request): Json<LoginRequest>,
) -> ApiResponseResult<LoginResponse> {
    response_from_result(login_user(state, request).await)
}

#[utoipa::path(
    get,
    path = "/api/v1/auth/me",
    tag = "auth",
    responses((status = 200, description = "Current user", body = ApiEnvelope<MeResponse>))
)]
pub async fn me(
    Extension(auth_context): Extension<AuthContext>,
    State(state): State<Arc<ServerState>>,
) -> ApiResponseResult<MeResponse> {
    response_from_result(current_user(state, auth_context.claims).await)
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/logout",
    tag = "auth",
    responses((status = 200, description = "Logout successful", body = ApiEnvelope<LogoutResponse>))
)]
pub async fn logout() -> ApiResponse<LogoutResponse> {
    api_ok(LogoutResponse {
        message: "Logout successful; discard the JWT on the client.",
    })
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/check-if-user-exists",
    tag = "auth",
    request_body = CheckIfUserExistsRequest,
    responses((status = 200, description = "Email existence", body = ApiEnvelope<CheckIfUserExistsResponse>))
)]
pub async fn check_if_user_exists(
    State(state): State<Arc<ServerState>>,
    Json(request): Json<CheckIfUserExistsRequest>,
) -> ApiResponseResult<CheckIfUserExistsResponse> {
    response_from_result(check_if_user_exists_service(state, request).await)
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/reset-password-request",
    tag = "auth",
    request_body = ResetPasswordRequest,
    responses((status = 200, description = "Password reset request processed", body = ApiEnvelope<ResetPasswordRequestResponse>))
)]
pub async fn reset_password_request(
    State(state): State<Arc<ServerState>>,
    Json(request): Json<ResetPasswordRequest>,
) -> ApiResponseResult<ResetPasswordRequestResponse> {
    response_from_result(request_password_reset(state, request).await)
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/reset-password",
    tag = "auth",
    request_body = ResetPasswordProcessRequest,
    responses((status = 200, description = "Password reset", body = ApiEnvelope<ResetPasswordResponse>))
)]
pub async fn reset_password(
    State(state): State<Arc<ServerState>>,
    Json(request): Json<ResetPasswordProcessRequest>,
) -> ApiResponseResult<ResetPasswordResponse> {
    response_from_result(reset_password_service(state, request).await)
}

#[utoipa::path(
    get,
    path = "/api/v1/auth/email-verification/challenge",
    tag = "auth",
    params(("email_validation_token_id" = Uuid, Query, description = "Email validation token")),
    responses((status = 200, description = "Email verification challenge", body = ApiEnvelope<EmailVerificationChallengeResponse>))
)]
pub async fn email_verification_challenge(
    State(state): State<Arc<ServerState>>,
    Query(token): Query<EmailValidationToken>,
    ConnectInfo(client_addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
) -> ApiResponseResult<EmailVerificationChallengeResponse> {
    response_from_result(
        issue_email_verification_challenge(state, token, Some(client_addr), user_agent(&headers))
            .await,
    )
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/verify-user-email",
    tag = "auth",
    request_body = VerifyEmailChallengeRequest,
    responses((status = 200, description = "Email verified", body = ApiEnvelope<VerifyEmailResponse>))
)]
pub async fn verify_user_email(
    State(state): State<Arc<ServerState>>,
    Json(request): Json<VerifyEmailChallengeRequest>,
) -> ApiResponseResult<VerifyEmailResponse> {
    response_from_result(verify_user_email_service(state, request).await)
}

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
    get,
    path = "/api/v1/users/{user_name}",
    tag = "user",
    params(("user_name" = String, Path, description = "Public username")),
    responses((status = 200, description = "Public user information", body = ApiEnvelope<PublicUserInfoResponse>))
)]
pub async fn get_user_info(
    State(state): State<Arc<ServerState>>,
    Path(user_name): Path<String>,
) -> ApiResponseResult<PublicUserInfoResponse> {
    response_from_result(public_user_info(state, user_name).await)
}

fn response_from_result<T>(result: ApiResult<T>) -> ApiResponseResult<T> {
    match result {
        Ok(response) => Ok(api_ok(response)),
        Err(error) => Err(error),
    }
}

fn user_agent(headers: &HeaderMap) -> Option<String> {
    let value = match headers.get(axum::http::header::USER_AGENT) {
        Some(value) => value,
        None => return None,
    };
    match value.to_str() {
        Ok(value) => Some(value.to_string()),
        Err(_) => None,
    }
}
