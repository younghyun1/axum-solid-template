use std::sync::Arc;

use axum::{
    Router,
    body::Body,
    http::Response,
    middleware::{from_fn, from_fn_with_state},
    response::IntoResponse,
    routing::{delete, get, post},
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
    controller::v1::{
        auth::{
            admin_add_email_verification_question_answer, admin_create_email_verification_question,
            admin_delete_email_verification_question,
            admin_delete_email_verification_question_answer, admin_email_verification_questions,
            check_if_user_exists, email_verification_challenge, get_user_info, login, logout, me,
            reset_password, reset_password_request, signup, verify_user_email,
        },
        healthcheck::healthcheck,
        reference_data::{
            countries as reference_countries,
            country_subdivisions as reference_country_subdivisions,
            languages as reference_languages,
        },
    },
    docs::api_doc::ApiDoc,
    error::{api_error::ApiError, code_error::CodeError},
    init::server_config::server_config::DeploymentEnvironment,
    init::state::server_state::ServerState,
    middleware::{
        api_timing::time_api_request,
        auth::{attach_optional_auth_context, require_admin, require_auth},
        request_response_logging::log_request_response,
    },
};

use super::static_assets::static_asset_handler;

const AUTH_RATE_LIMIT_REPLENISHED_EVERY_MILLISECONDS: u64 = 63;
const AUTH_RATE_LIMIT_BURST_SIZE: u32 = 1024;
const API_V1_PREFIX: &str = "/api/v1";

/// Builds the root Axum router with API v1 routes, Swagger UI, static assets, and middleware.
///
/// # Arguments
/// * `state` -
/// # Returns
/// Returns the value produced by this function.
pub fn build_router(state: Arc<ServerState>) -> Router {
    let cors_layer = cors_layer_for_environment(state.server_config.deployment_environment);
    let api_v1_router = build_api_v1_router(state);

    let swagger_router = SwaggerUi::new("/api/v1/swagger-ui")
        .url("/api/v1/api-docs/openapi.json", ApiDoc::openapi());

    Router::new()
        .nest(API_V1_PREFIX, api_v1_router)
        .merge(swagger_router)
        .fallback(static_asset_handler)
        .layer(cors_layer)
        .layer(CompressionLayer::new().gzip(true).zstd(true))
        .layer(from_fn(log_request_response))
}

/// Composes all public/auth/admin API v1 routes and wires auth context/timing middleware.
///
/// # Arguments
/// * `state` -
/// # Returns
/// Returns the value produced by this function.
fn build_api_v1_router(state: Arc<ServerState>) -> Router {
    let public_auth_router = apply_auth_rate_limit(build_public_auth_router());
    let protected_auth_router = build_protected_auth_router(state.clone());
    let admin_router = build_admin_router(state.clone());

    Router::new()
        .route("/healthcheck", get(healthcheck))
        .route("/reference/countries", get(reference_countries))
        .route("/reference/languages", get(reference_languages))
        .route(
            "/reference/countries/{country_code}/subdivisions",
            get(reference_country_subdivisions),
        )
        .merge(public_auth_router)
        .merge(protected_auth_router)
        .merge(admin_router)
        .layer(from_fn_with_state(
            state.clone(),
            attach_optional_auth_context,
        ))
        .layer(from_fn(time_api_request))
        .with_state(state)
}

/// Builds routes that do not require authentication (signup/login/reset/etc.).
///
/// # Returns
/// Returns the value produced by this function.
fn build_public_auth_router() -> Router<Arc<ServerState>> {
    Router::new()
        .route("/auth/signup", post(signup))
        .route("/auth/login", post(login))
        .route("/auth/check-if-user-exists", post(check_if_user_exists))
        .route("/auth/reset-password-request", post(reset_password_request))
        .route("/auth/reset-password", post(reset_password))
        .route(
            "/auth/email-verification/challenge",
            get(email_verification_challenge),
        )
        .route("/auth/verify-user-email", post(verify_user_email))
        .route("/users/{user_name}", get(get_user_info))
}

/// Builds routes that require an attached `AuthContext`.
///
/// # Arguments
/// * `state` -
/// # Returns
/// Returns the value produced by this function.
fn build_protected_auth_router(state: Arc<ServerState>) -> Router<Arc<ServerState>> {
    Router::new()
        .route("/auth/me", get(me))
        .route("/auth/logout", post(logout))
        .layer(from_fn_with_state(state, require_auth))
}

/// Builds admin-only routes for email verification question management.
///
/// # Arguments
/// * `state` -
/// # Returns
/// Returns the value produced by this function.
fn build_admin_router(state: Arc<ServerState>) -> Router<Arc<ServerState>> {
    Router::new()
        .route(
            "/admin/email-verification/questions",
            get(admin_email_verification_questions).post(admin_create_email_verification_question),
        )
        .route(
            "/admin/email-verification/questions/{question_id}",
            delete(admin_delete_email_verification_question),
        )
        .route(
            "/admin/email-verification/questions/{question_id}/answers",
            post(admin_add_email_verification_question_answer),
        )
        .route(
            "/admin/email-verification/questions/{question_id}/answers/{answer_id}",
            delete(admin_delete_email_verification_question_answer),
        )
        .layer(from_fn_with_state(state, require_admin))
}

/// Applies permissive CORS locally/development, and constrained CORS in production.
///
/// # Arguments
/// * `deployment_environment` -
/// # Returns
/// Returns the value produced by this function.
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

/// Applies rate limiting middleware to public auth endpoints and falls back gracefully on config failure.
///
/// # Arguments
/// * `public_auth_router` -
/// # Returns
/// Returns the value produced by this function.
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

/// Translates governor errors into API error responses and attaches retry headers where available.
///
/// # Arguments
/// * `error` -
/// # Returns
/// Returns the value produced by this function.
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
