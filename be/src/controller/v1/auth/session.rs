use std::sync::Arc;

use axum::{Json, extract::State, http::HeaderMap, response::Response};

use crate::{
    dto::{
        api_response::ApiEnvelope,
        auth::{
            request::LoginRequest,
            response::{LoginResponse, LogoutResponse, RefreshSessionResponse},
        },
    },
    error::{api_error::ApiError, code_error::CodeError},
    init::state::server_state::ServerState,
    service::auth::{
        login::login_user,
        session::{logout_auth_session, refresh_auth_session},
    },
    util::auth::cookie::refresh_session_from_headers,
};

use super::support::{response_with_auth_cookies, response_with_cleared_auth_cookies};

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
) -> Result<Response, ApiError> {
    let issued = match login_user(state.clone(), request).await {
        Ok(issued) => issued,
        Err(error) => return Err(error),
    };

    response_with_auth_cookies(&state, issued)
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/refresh",
    tag = "auth",
    responses((status = 200, description = "Session refreshed", body = ApiEnvelope<RefreshSessionResponse>))
)]
pub async fn refresh(
    State(state): State<Arc<ServerState>>,
    headers: HeaderMap,
) -> Result<Response, ApiError> {
    let refresh_token = match refresh_session_from_headers(&headers) {
        Some(refresh_token) => refresh_token,
        None => return Err(ApiError::new(CodeError::REFRESH_SESSION_INVALID)),
    };
    let issued = match refresh_auth_session(state.clone(), refresh_token).await {
        Ok(issued) => issued,
        Err(error) => return Err(error),
    };

    response_with_auth_cookies(&state, issued)
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/logout",
    tag = "auth",
    responses((status = 200, description = "Logout successful", body = ApiEnvelope<LogoutResponse>))
)]
pub async fn logout(
    State(state): State<Arc<ServerState>>,
    headers: HeaderMap,
) -> Result<Response, ApiError> {
    let refresh_token = refresh_session_from_headers(&headers);
    let response = match logout_auth_session(state.clone(), refresh_token).await {
        Ok(response) => response,
        Err(error) => return Err(error),
    };

    response_with_cleared_auth_cookies(&state, response)
}
