use axum::{
    http::HeaderMap,
    response::{IntoResponse, Response},
};
use serde::Serialize;

use crate::{
    dto::api_response::{ApiResponseResult, ApiResult, api_ok},
    error::{api_error::ApiError, code_error::CodeError},
    init::state::server_state::ServerState,
    service::auth::session::IssuedAuthSession,
    util::auth::cookie::{
        append_set_cookie_headers, auth_cookie_headers, clear_auth_cookie_headers,
    },
};

pub(super) fn response_from_result<T>(result: ApiResult<T>) -> ApiResponseResult<T> {
    match result {
        Ok(response) => Ok(api_ok(response)),
        Err(error) => Err(error),
    }
}

pub(super) fn response_with_auth_cookies<T>(
    state: &ServerState,
    issued: IssuedAuthSession<T>,
) -> Result<Response, ApiError>
where
    T: Serialize,
{
    let cookie_headers = match auth_cookie_headers(
        &state.server_config.cookie_config,
        &issued.access_token,
        state.server_config.jwt_config.access_token_duration,
        &issued.refresh_token,
        state.server_config.jwt_config.refresh_token_duration,
    ) {
        Ok(cookie_headers) => cookie_headers,
        Err(error) => return Err(ApiError::from_source(CodeError::INTERNAL_ERROR, error)),
    };

    let mut response = api_ok(issued.response).into_response();
    append_set_cookie_headers(response.headers_mut(), cookie_headers);
    Ok(response)
}

pub(super) fn response_with_cleared_auth_cookies<T>(
    state: &ServerState,
    payload: T,
) -> Result<Response, ApiError>
where
    T: Serialize,
{
    let cookie_headers = match clear_auth_cookie_headers(&state.server_config.cookie_config) {
        Ok(cookie_headers) => cookie_headers,
        Err(error) => return Err(ApiError::from_source(CodeError::INTERNAL_ERROR, error)),
    };

    let mut response = api_ok(payload).into_response();
    append_set_cookie_headers(response.headers_mut(), cookie_headers);
    Ok(response)
}

pub(super) fn user_agent(headers: &HeaderMap) -> Option<String> {
    let value = match headers.get(axum::http::header::USER_AGENT) {
        Some(value) => value,
        None => return None,
    };
    match value.to_str() {
        Ok(value) => Some(value.to_string()),
        Err(_) => None,
    }
}
