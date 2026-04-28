use std::sync::Arc;

use axum::{
    Router,
    middleware::{from_fn, from_fn_with_state},
    routing::{get, post},
};
use tower_http::{compression::CompressionLayer, cors::CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    controller::{
        auth::{
            check_if_user_exists, get_user_info, is_superuser, login, logout, me, reset_password,
            reset_password_request, signup, verify_user_email,
        },
        healthcheck::healthcheck,
    },
    docs::api_doc::ApiDoc,
    init::server_config::server_config::DeploymentEnvironment,
    init::state::server_state::ServerState,
    middleware::auth::require_auth,
    middleware::request_response_logging::log_request_response,
};

use super::static_assets::static_asset_handler;

pub fn build_router(state: Arc<ServerState>) -> Router {
    let cors_layer = cors_layer_for_environment(state.server_config.deployment_environment);

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

    let protected_auth_router = Router::new()
        .route("/api/auth/me", get(me))
        .route("/api/auth/logout", post(logout))
        .route("/api/auth/is-superuser", get(is_superuser))
        .route("/api/v1/auth/me", get(me))
        .route("/api/v1/auth/logout", post(logout))
        .route("/api/v1/auth/is-superuser", get(is_superuser))
        .layer(from_fn_with_state(state.clone(), require_auth));

    let api_router = Router::new()
        .route("/healthcheck", get(healthcheck))
        .merge(public_auth_router)
        .merge(protected_auth_router)
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
