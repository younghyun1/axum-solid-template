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
    /// Checks whether the token includes a specific role.
    ///
    /// # Arguments
    /// * `self` -
    /// * `role_type` -
    /// # Returns
    /// Returns the value produced by this function.
    pub fn has_role(&self, role_type: RoleType) -> bool {
        self.claims.has_role(role_type)
    }

    /// Checks whether the token role is at least the requested minimum.
    ///
    /// # Arguments
    /// * `self` -
    /// * `minimum_role` -
    /// # Returns
    /// Returns the value produced by this function.
    pub fn has_min_role(&self, minimum_role: RoleType) -> bool {
        self.claims.has_min_role(minimum_role)
    }

    /// Checks for admin role.
    ///
    /// # Arguments
    /// * `self` -
    /// # Returns
    /// Returns the value produced by this function.
    pub fn is_admin(&self) -> bool {
        self.claims.is_admin()
    }

    /// Checks for moderator role.
    ///
    /// # Arguments
    /// * `self` -
    /// # Returns
    /// Returns the value produced by this function.
    pub fn is_moderator(&self) -> bool {
        self.claims.is_moderator()
    }

    /// Checks for service provider role.
    ///
    /// # Arguments
    /// * `self` -
    /// # Returns
    /// Returns the value produced by this function.
    pub fn is_service_provider(&self) -> bool {
        self.claims.is_service_provider()
    }

    /// Checks for user/client role.
    ///
    /// # Arguments
    /// * `self` -
    /// # Returns
    /// Returns the value produced by this function.
    pub fn is_user_client(&self) -> bool {
        self.claims.is_user_client()
    }

    /// Checks for guest role.
    ///
    /// # Arguments
    /// * `self` -
    /// # Returns
    /// Returns the value produced by this function.
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

/// Validates the bearer token and decodes claims into an `AuthContext`.
///
/// # Arguments
/// * `state` -
/// * `token` -
/// # Returns
/// A `Result`, either containing the function output or an error.
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

/// Detects whether an `AuthContext` extension is already attached to request metadata.
///
/// # Arguments
/// * `request` -
/// # Returns
/// Returns the value produced by this function.
fn auth_context_attached(request: &Request<Body>) -> bool {
    request.extensions().get::<AuthContext>().is_some()
}

/// Parses and validates the `Authorization` header as `Bearer <token>`.
///
/// # Arguments
/// * `request` -
/// # Returns
/// Returns the value produced by this function.
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
