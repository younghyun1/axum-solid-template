use std::sync::Arc;

use axum::{
    Extension, Json,
    extract::{Path, Query, State},
};

use crate::{
    dto::{
        api_response::{ApiEnvelope, ApiResponse, ApiResponseResult, ApiResult},
        auth::{
            request::{
                CheckIfUserExistsRequest, EmailValidationToken, LoginRequest,
                ResetPasswordProcessRequest, ResetPasswordRequest, SignupRequest,
            },
            response::{
                CheckIfUserExistsResponse, LoginResponse, LogoutResponse, MeResponse,
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
        verification::verify_user_email as verify_user_email_service,
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
    path = "/api/v1/auth/verify-user-email",
    tag = "auth",
    params(("email_validation_token_id" = Uuid, Query, description = "Email validation token")),
    responses((status = 200, description = "Email verified", body = ApiEnvelope<VerifyEmailResponse>))
)]
pub async fn verify_user_email(
    State(state): State<Arc<ServerState>>,
    Query(token): Query<EmailValidationToken>,
) -> ApiResponseResult<VerifyEmailResponse> {
    response_from_result(verify_user_email_service(state, token).await)
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
