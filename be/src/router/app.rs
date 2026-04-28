use std::sync::Arc;

use axum::{Router, middleware::from_fn, routing::get};
use tower_http::{compression::CompressionLayer, cors::CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    controller::healthcheck::healthcheck, docs::api_doc::ApiDoc,
    init::server_config::server_config::DeploymentEnvironment,
    init::state::server_state::ServerState,
    middleware::request_response_logging::log_request_response,
};

use super::static_assets::static_asset_handler;

pub fn build_router(state: Arc<ServerState>) -> Router {
    let cors_layer = cors_layer_for_environment(state.server_config.deployment_environment);

    let api_router = Router::new()
        .route("/healthcheck", get(healthcheck))
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
