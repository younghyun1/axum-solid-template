use axum::{
    body::Bytes,
    http::{StatusCode, header},
    response::IntoResponse,
};

#[utoipa::path(
    get,
    path = "/healthcheck",
    tag = "server",
    responses(
        (status = 200, description = "Server is accepting traffic", content_type = "application/octet-stream")
    )
)]
pub async fn healthcheck() -> impl IntoResponse {
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/octet-stream")],
        Bytes::from_static(&[1_u8]),
    )
}
