use std::sync::Arc;

use axum::{Router, routing::get};
use tower_http::{compression::CompressionLayer, cors::CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    controller::healthcheck::healthcheck, docs::api_doc::ApiDoc,
    init::state::server_state::ServerState,
};

use super::static_assets::static_asset_handler;

pub fn build_router(state: Arc<ServerState>) -> Router {
    let api_router = Router::new()
        .route("/healthcheck", get(healthcheck))
        .with_state(state);

    let swagger_router =
        SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi());

    Router::new()
        .merge(api_router)
        .merge(swagger_router)
        .layer(CorsLayer::very_permissive())
        .layer(CompressionLayer::new().gzip(true).zstd(true))
        .fallback(static_asset_handler)
}
