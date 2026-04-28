use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(crate::controller::healthcheck::healthcheck),
    components(schemas(
        crate::dto::api_response::ApiErrorBody,
        crate::dto::api_response::ApiErrorLevel,
        crate::dto::api_response::ApiMeta,
        crate::dto::healthcheck::HealthcheckResponse
    )),
    tags((name = "server", description = "Server health and runtime endpoints"))
)]
pub struct ApiDoc;
