use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(crate::controller::healthcheck::healthcheck),
    tags((name = "server", description = "Server health and runtime endpoints"))
)]
pub struct ApiDoc;
