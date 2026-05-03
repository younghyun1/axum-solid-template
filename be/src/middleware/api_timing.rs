use axum::{body::Body, http::Request, middleware::Next, response::Response};

use crate::dto::api_response::ApiTimer;

/// Starts an API timer, executes the downstream handler in that timer scope, and returns the response.
///
/// # Arguments
/// * `request` -
/// * `next` -
/// # Returns
/// Returns the value produced by this function.
pub async fn time_api_request(request: Request<Body>, next: Next) -> Response<Body> {
    let timer = ApiTimer::start();
    timer.scope(next.run(request)).await
}
