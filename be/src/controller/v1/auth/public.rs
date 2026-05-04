use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};

use crate::{
    dto::{
        api_response::{ApiEnvelope, ApiResponseResult},
        auth::{
            request::{
                CheckIfUserExistsRequest, ResetPasswordProcessRequest, ResetPasswordRequest,
                SignupRequest,
            },
            response::{
                CheckIfUserExistsResponse, PublicUserInfoResponse, ResetPasswordRequestResponse,
                ResetPasswordResponse, SignupResponse,
            },
        },
    },
    init::state::server_state::ServerState,
    service::auth::{
        password_reset::{request_password_reset, reset_password as reset_password_service},
        signup::signup_user,
        user::{check_if_user_exists as check_if_user_exists_service, public_user_info},
    },
};

use super::support::response_from_result;

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
