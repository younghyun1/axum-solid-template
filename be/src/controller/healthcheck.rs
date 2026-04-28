use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use serde::Serialize;
use utoipa::ToSchema;

use crate::init::state::server_state::ServerState;

#[derive(Debug, Serialize, ToSchema)]
pub struct HealthcheckResponse {
    status: &'static str,
    deployment_environment: &'static str,
}

#[utoipa::path(
    get,
    path = "/healthcheck",
    tag = "server",
    responses(
        (status = 200, description = "Server is accepting traffic", body = HealthcheckResponse)
    )
)]
pub async fn healthcheck(
    State(state): State<Arc<ServerState>>,
) -> (StatusCode, Json<HealthcheckResponse>) {
    (
        StatusCode::OK,
        Json(HealthcheckResponse {
            status: "ok",
            deployment_environment: state.server_config.deployment_environment.as_str(),
        }),
    )
}
