use crate::{
    dto::{
        api_response::{ApiEnvelope, ApiResponse},
        healthcheck::HealthcheckResponse,
    },
    error::prelude::api_ok,
};

#[utoipa::path(
    get,
    path = "/api/v1/healthcheck",
    tag = "server",
    responses(
        (
            status = 200,
            description = "Server is accepting traffic",
            body = ApiEnvelope<HealthcheckResponse>
        )
    )
)]
/// Return the current liveness state for load-balancer and operations checks.
///
/// # Returns
/// `ApiResponse<HealthcheckResponse>` indicating that the server is accepting traffic.
pub async fn healthcheck() -> ApiResponse<HealthcheckResponse> {
    api_ok(HealthcheckResponse::accepting_traffic())
}
