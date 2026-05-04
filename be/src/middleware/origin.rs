use std::sync::Arc;

use axum::{
    body::Body,
    extract::State,
    http::{Method, Request},
    middleware::Next,
    response::Response,
};
use tracing::warn;

use crate::{
    error::{api_error::ApiError, code_error::CodeError},
    init::state::server_state::ServerState,
};

pub async fn validate_request_origin(
    State(state): State<Arc<ServerState>>,
    request: Request<Body>,
    next: Next,
) -> Result<Response<Body>, ApiError> {
    if matches!(
        request.method(),
        &Method::GET | &Method::HEAD | &Method::OPTIONS | &Method::TRACE
    ) {
        return Ok(next.run(request).await);
    }

    let origin = match request.headers().get(axum::http::header::ORIGIN) {
        Some(origin) => origin,
        None => return Ok(next.run(request).await),
    };
    let origin = match origin.to_str() {
        Ok(origin) => origin,
        Err(_) => return Err(ApiError::new(CodeError::ORIGIN_NOT_ALLOWED)),
    };

    if state.server_config.cors_config.is_origin_allowed(origin) {
        return Ok(next.run(request).await);
    }

    warn!(origin = %origin, "Rejected unsafe request from disallowed origin");
    Err(ApiError::new(CodeError::ORIGIN_NOT_ALLOWED))
}
