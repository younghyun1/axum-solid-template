use axum::{body::Body, http::Request, middleware::Next, response::Response};

use crate::dto::api_response::ApiTimer;

pub async fn time_api_request(request: Request<Body>, next: Next) -> Response<Body> {
    let timer = ApiTimer::start();
    timer.scope(next.run(request)).await
}
