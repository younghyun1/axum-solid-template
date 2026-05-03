use std::{
    fmt,
    time::{Duration, Instant},
};

use axum::{
    body::Body,
    http::{HeaderMap, HeaderName, Request, Response, StatusCode, Version, header},
    middleware::Next,
};
use tracing::{error, info, warn};

use crate::dto::api_response::iso_duration_from_duration;

#[derive(Debug, Clone, Copy)]
struct OptionalStr<'a>(Option<&'a str>);

impl fmt::Display for OptionalStr<'_> {
    /// Formats optional header and query values into empty string when absent.
    ///
    /// # Arguments
    /// * `self` -
    /// * `formatter` -
    /// # Returns
    /// Returns the value produced by this function.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            Some(value) => formatter.write_str(value),
            None => formatter.write_str(""),
        }
    }
}

/// Logs request metadata, runs downstream handler, and logs response metadata with duration.
///
/// # Arguments
/// * `request` -
/// * `next` -
/// # Returns
/// Returns the value produced by this function.
#[allow(clippy::manual_map)]
pub async fn log_request_response(request: Request<Body>, next: Next) -> Response<Body> {
    let started_at = Instant::now();
    let (parts, body) = request.into_parts();

    let method = parts.method.clone();
    let path = parts.uri.path().to_string();
    let query = match parts.uri.query() {
        Some(value) => Some(value.to_string()),
        None => None,
    };
    let version = parts.version;
    let user_agent = header_value(&parts.headers, header::USER_AGENT);
    let request_content_type = header_value(&parts.headers, header::CONTENT_TYPE);
    let request_content_length = header_value(&parts.headers, header::CONTENT_LENGTH);

    log_incoming_request(
        &method,
        &path,
        optional_string_value(&query),
        version,
        user_agent,
        request_content_type,
        request_content_length,
    );

    let request = Request::from_parts(parts, body);
    let response = next.run(request).await;
    let status = response.status();
    let response_content_type = header_value(response.headers(), header::CONTENT_TYPE);
    let response_content_length = header_value(response.headers(), header::CONTENT_LENGTH);
    let duration = started_at.elapsed();

    log_outgoing_response(
        &method,
        &path,
        optional_string_value(&query),
        version,
        status,
        duration,
        response_content_type,
        response_content_length,
    );

    response
}

#[allow(clippy::too_many_arguments)]
fn log_incoming_request(
    method: &axum::http::Method,
    path: &str,
    query: OptionalStr<'_>,
    version: Version,
    user_agent: OptionalStr<'_>,
    request_content_type: OptionalStr<'_>,
    request_content_length: OptionalStr<'_>,
) {
    info!(
        request_method = %method,
        request_path = path,
        request_query = %query,
        http_version = ?version,
        user_agent = %user_agent,
        request_content_type = %request_content_type,
        request_content_length = %request_content_length,
        "HTTP request received"
    );
}

#[allow(clippy::too_many_arguments)]
fn log_outgoing_response(
    method: &axum::http::Method,
    path: &str,
    query: OptionalStr<'_>,
    version: Version,
    status: StatusCode,
    duration: Duration,
    response_content_type: OptionalStr<'_>,
    response_content_length: OptionalStr<'_>,
) {
    let duration_iso = iso_duration_from_duration(duration);
    let status_code = status.as_u16();

    if status.is_server_error() {
        error!(
            request_method = %method,
            request_path = path,
            request_query = %query,
            http_version = ?version,
            http_status_code = status_code,
            response_content_type = %response_content_type,
            response_content_length = %response_content_length,
            processing_duration = %duration_iso,
            "HTTP response completed"
        );
        return;
    }

    if status.is_client_error() {
        warn!(
            request_method = %method,
            request_path = path,
            request_query = %query,
            http_version = ?version,
            http_status_code = status_code,
            response_content_type = %response_content_type,
            response_content_length = %response_content_length,
            processing_duration = %duration_iso,
            "HTTP response completed"
        );
        return;
    }

    info!(
        request_method = %method,
        request_path = path,
        request_query = %query,
        http_version = ?version,
        http_status_code = status_code,
        response_content_type = %response_content_type,
        response_content_length = %response_content_length,
        processing_duration = %duration_iso,
        "HTTP response completed"
    );
}

/// Extracts a header by name and returns a printable optional wrapper.
///
/// # Arguments
/// * `headers` -
/// * `header_name` -
/// # Returns
/// Returns the value produced by this function.
fn header_value(headers: &HeaderMap, header_name: HeaderName) -> OptionalStr<'_> {
    match headers.get(header_name) {
        Some(value) => match value.to_str() {
            Ok(parsed) => OptionalStr(Some(parsed)),
            Err(_) => OptionalStr(Some("<invalid>")),
        },
        None => OptionalStr(None),
    }
}

/// Converts an `Option<String>` into the internal optional string wrapper.
///
/// # Arguments
/// * `value` -
/// # Returns
/// Returns the value produced by this function.
fn optional_string_value(value: &Option<String>) -> OptionalStr<'_> {
    match value {
        Some(inner) => OptionalStr(Some(inner.as_str())),
        None => OptionalStr(None),
    }
}
