use std::sync::Arc;

use axum::{
    Router,
    body::Body,
    http::Response,
    middleware::{from_fn, from_fn_with_state},
    response::IntoResponse,
    routing::{get, post},
};
use tower_governor::{
    GovernorError, GovernorLayer, governor::GovernorConfigBuilder,
    key_extractor::SmartIpKeyExtractor,
};
use tower_http::{compression::CompressionLayer, cors::CorsLayer};
use tracing::{error, warn};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    controller::{
        auth::{
            check_if_user_exists, get_user_info, login, logout, me, reset_password,
            reset_password_request, signup, verify_user_email,
        },
        healthcheck::healthcheck,
    },
    docs::api_doc::ApiDoc,
    error::{api_error::ApiError, code_error::CodeError},
    init::server_config::server_config::DeploymentEnvironment,
    init::state::server_state::ServerState,
    middleware::auth::{attach_optional_auth_context, require_auth},
    middleware::request_response_logging::log_request_response,
};

use super::static_assets::static_asset_handler;

const AUTH_RATE_LIMIT_REPLENISHED_EVERY_MILLISECONDS: u64 = 63;
const AUTH_RATE_LIMIT_BURST_SIZE: u32 = 1024;

pub fn build_router(state: Arc<ServerState>) -> Router {
    let cors_layer = cors_layer_for_environment(state.server_config.deployment_environment);

    // Keep unversioned and v1 routes together until the template has a versioning policy.
    let public_auth_router = Router::new()
        .route("/api/auth/signup", post(signup))
        .route("/api/auth/login", post(login))
        .route("/api/auth/check-if-user-exists", post(check_if_user_exists))
        .route(
            "/api/auth/reset-password-request",
            post(reset_password_request),
        )
        .route("/api/auth/reset-password", post(reset_password))
        .route("/api/auth/verify-user-email", get(verify_user_email))
        .route("/api/users/{user_name}", get(get_user_info))
        .route("/api/v1/auth/signup", post(signup))
        .route("/api/v1/auth/login", post(login))
        .route(
            "/api/v1/auth/check-if-user-exists",
            post(check_if_user_exists),
        )
        .route(
            "/api/v1/auth/reset-password-request",
            post(reset_password_request),
        )
        .route("/api/v1/auth/reset-password", post(reset_password))
        .route("/api/v1/auth/verify-user-email", get(verify_user_email))
        .route("/api/v1/users/{user_name}", get(get_user_info));
    let public_auth_router = apply_auth_rate_limit(public_auth_router);

    let protected_auth_router = Router::new()
        .route("/api/auth/me", get(me))
        .route("/api/auth/logout", post(logout))
        .route("/api/v1/auth/me", get(me))
        .route("/api/v1/auth/logout", post(logout))
        .layer(from_fn_with_state(state.clone(), require_auth));

    let api_router = Router::new()
        .route("/healthcheck", get(healthcheck))
        .merge(public_auth_router)
        .merge(protected_auth_router)
        .layer(from_fn_with_state(
            state.clone(),
            attach_optional_auth_context,
        ))
        .with_state(state);

    let swagger_router =
        SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi());

    Router::new()
        .merge(api_router)
        .merge(swagger_router)
        .fallback(static_asset_handler)
        .layer(cors_layer)
        .layer(CompressionLayer::new().gzip(true).zstd(true))
        .layer(from_fn(log_request_response))
}

fn cors_layer_for_environment(deployment_environment: DeploymentEnvironment) -> CorsLayer {
    match deployment_environment {
        DeploymentEnvironment::Local | DeploymentEnvironment::Development => {
            CorsLayer::very_permissive()
        }
        DeploymentEnvironment::Production | DeploymentEnvironment::ProductionDockerized => {
            CorsLayer::new()
        }
    }
}

fn apply_auth_rate_limit(public_auth_router: Router<Arc<ServerState>>) -> Router<Arc<ServerState>> {
    let mut builder = GovernorConfigBuilder::default();
    let config = builder
        .per_millisecond(AUTH_RATE_LIMIT_REPLENISHED_EVERY_MILLISECONDS)
        .burst_size(AUTH_RATE_LIMIT_BURST_SIZE)
        .key_extractor(SmartIpKeyExtractor)
        .use_headers()
        .finish();

    match config {
        Some(config) => public_auth_router
            .layer(GovernorLayer::new(config).error_handler(rate_limit_error_response)),
        None => {
            error!(
                rate_limit_replenished_every_milliseconds =
                    AUTH_RATE_LIMIT_REPLENISHED_EVERY_MILLISECONDS,
                rate_limit_burst_size = AUTH_RATE_LIMIT_BURST_SIZE,
                "Failed to build auth rate limiter"
            );
            public_auth_router
        }
    }
}

fn rate_limit_error_response(error: GovernorError) -> Response<Body> {
    match error {
        GovernorError::TooManyRequests { wait_time, headers } => {
            warn!(
                retry_after_seconds = wait_time,
                "Rate limit exceeded for auth route"
            );
            let mut response = ApiError::new(CodeError::RATE_LIMITED).into_response();
            if let Some(headers) = headers {
                response.headers_mut().extend(headers);
            }
            response
        }
        GovernorError::UnableToExtractKey => {
            error!("Rate limiter could not extract a client key");
            ApiError::new(CodeError::INTERNAL_ERROR).into_response()
        }
        GovernorError::Other { code, msg, headers } => {
            error!(
                http_status_code = code.as_u16(),
                message = ?msg,
                "Rate limiter returned an unexpected error"
            );
            let mut response = ApiError::new(CodeError::INTERNAL_ERROR).into_response();
            if let Some(headers) = headers {
                response.headers_mut().extend(headers);
            }
            response
        }
    }
}
