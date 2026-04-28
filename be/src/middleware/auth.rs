use std::sync::Arc;

use axum::{body::Body, extract::State, http::Request, middleware::Next, response::Response};
use tracing::info;

use crate::{
    domain::auth::jwt::AccessTokenClaims,
    error::{api_error::ApiError, code_error::CodeError},
    init::server_config::jwt_config::jwt_config::{
        JWT_AUTHORIZATION_HEADER_NAME, JWT_BEARER_SCHEME,
    },
    init::state::server_state::ServerState,
    util::auth::jwt::decode_access_token,
};

#[derive(Debug, Clone)]
pub struct AuthContext {
    pub claims: AccessTokenClaims,
}

pub async fn require_auth(
    State(state): State<Arc<ServerState>>,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response<Body>, ApiError> {
    let token = match bearer_token(&request) {
        Some(token) => token,
        None => return Err(ApiError::new(CodeError::UNAUTHORIZED)),
    };

    let claims = match decode_access_token(&state.server_config.jwt_config, token) {
        Ok(claims) => claims,
        Err(error) => {
            info!(error = %error, "JWT access token rejected");
            return Err(ApiError::new(CodeError::JWT_INVALID));
        }
    };

    request.extensions_mut().insert(AuthContext { claims });
    Ok(next.run(request).await)
}

fn bearer_token(request: &Request<Body>) -> Option<&str> {
    let value = match request.headers().get(JWT_AUTHORIZATION_HEADER_NAME) {
        Some(value) => value,
        None => return None,
    };
    let parsed = match value.to_str() {
        Ok(parsed) => parsed,
        Err(_) => return None,
    };
    let trimmed = parsed.trim();
    let scheme_len = JWT_BEARER_SCHEME.len();

    if trimmed.len() <= scheme_len {
        return None;
    }

    let (scheme, rest) = trimmed.split_at(scheme_len);
    if !scheme.eq_ignore_ascii_case(JWT_BEARER_SCHEME) {
        return None;
    }

    let token = rest.trim();
    if token.is_empty() {
        return None;
    }

    Some(token)
}
