use std::sync::Arc;

use axum::{body::Body, extract::State, http::Request, middleware::Next, response::Response};
use tracing::info;

use crate::{
    domain::auth::{jwt::AccessTokenClaims, role::RoleType},
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

impl AuthContext {
    pub fn has_role(&self, role_type: RoleType) -> bool {
        self.claims.has_role(role_type)
    }

    pub fn has_min_role(&self, minimum_role: RoleType) -> bool {
        self.claims.has_min_role(minimum_role)
    }

    pub fn is_admin(&self) -> bool {
        self.claims.is_admin()
    }

    pub fn is_moderator(&self) -> bool {
        self.claims.is_moderator()
    }

    pub fn is_service_provider(&self) -> bool {
        self.claims.is_service_provider()
    }

    pub fn is_user_client(&self) -> bool {
        self.claims.is_user_client()
    }

    pub fn is_guest(&self) -> bool {
        self.claims.is_guest()
    }
}

pub async fn attach_optional_auth_context(
    State(state): State<Arc<ServerState>>,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response<Body>, ApiError> {
    if auth_context_attached(&request) {
        return Ok(next.run(request).await);
    }

    let token = match bearer_token(&request) {
        Some(token) => token,
        None => return Ok(next.run(request).await),
    };

    let auth_context = match auth_context_from_token(&state, token) {
        Ok(auth_context) => auth_context,
        Err(error) => return Err(error),
    };

    request.extensions_mut().insert(auth_context);
    Ok(next.run(request).await)
}

pub async fn require_auth(
    State(state): State<Arc<ServerState>>,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response<Body>, ApiError> {
    if auth_context_attached(&request) {
        return Ok(next.run(request).await);
    }

    let token = match bearer_token(&request) {
        Some(token) => token,
        None => return Err(ApiError::new(CodeError::UNAUTHORIZED)),
    };

    let auth_context = match auth_context_from_token(&state, token) {
        Ok(auth_context) => auth_context,
        Err(error) => return Err(error),
    };

    request.extensions_mut().insert(auth_context);
    Ok(next.run(request).await)
}

pub async fn require_admin(
    State(state): State<Arc<ServerState>>,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response<Body>, ApiError> {
    if !auth_context_attached(&request) {
        let token = match bearer_token(&request) {
            Some(token) => token,
            None => return Err(ApiError::new(CodeError::UNAUTHORIZED)),
        };

        let auth_context = match auth_context_from_token(&state, token) {
            Ok(auth_context) => auth_context,
            Err(error) => return Err(error),
        };

        request.extensions_mut().insert(auth_context);
    }

    let auth_context = match request.extensions().get::<AuthContext>() {
        Some(auth_context) => auth_context,
        None => return Err(ApiError::new(CodeError::UNAUTHORIZED)),
    };

    if !auth_context.is_admin() {
        return Err(ApiError::new(CodeError::ADMIN_REQUIRED));
    }

    Ok(next.run(request).await)
}

fn auth_context_from_token(state: &ServerState, token: &str) -> Result<AuthContext, ApiError> {
    let claims = match decode_access_token(&state.server_config.jwt_config, token) {
        Ok(claims) => claims,
        Err(error) => {
            info!(error = %error, "JWT access token rejected");
            return Err(ApiError::new(CodeError::JWT_INVALID));
        }
    };

    Ok(AuthContext { claims })
}

fn auth_context_attached(request: &Request<Body>) -> bool {
    request.extensions().get::<AuthContext>().is_some()
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
