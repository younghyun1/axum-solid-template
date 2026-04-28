use crate::{
    dto::{
        api_response::{ApiEnvelope, ApiResponse},
        healthcheck::HealthcheckResponse,
    },
    error::prelude::api_ok,
};

#[utoipa::path(
    get,
    path = "/healthcheck",
    tag = "server",
    responses(
        (
            status = 200,
            description = "Server is accepting traffic",
            body = ApiEnvelope<HealthcheckResponse>
        )
    )
)]
pub async fn healthcheck() -> ApiResponse<HealthcheckResponse> {
    api_ok(HealthcheckResponse::accepting_traffic())
}
