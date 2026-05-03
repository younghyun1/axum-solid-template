use std::{
    future::Future,
    time::{Duration, Instant},
};

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use chrono::{SecondsFormat, Utc};
use serde::Serialize;
use utoipa::ToSchema;

use crate::error::code_error::CodeError;

pub type ApiResult<T> = Result<T, crate::error::api_error::ApiError>;
pub type ApiResponseResult<T> = Result<ApiResponse<T>, crate::error::api_error::ApiError>;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ApiEnvelope<T, M = ApiMeta> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<ApiErrorBody>,
    pub meta: M,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ApiMeta {
    pub timestamp: String,
    pub processing_duration: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Copy)]
pub struct ApiTimer {
    started_at: Instant,
}

tokio::task_local! {
    static CURRENT_API_TIMER: ApiTimer;
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ApiErrorBody {
    pub error_code: u8,
    pub error_level: ApiErrorLevel,
    pub error_message: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_detail: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ApiErrorLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

#[derive(Debug, Clone)]
pub struct ApiResponse<T> {
    status_code: StatusCode,
    body: ApiEnvelope<T, ApiMeta>,
}

pub trait IntoApiResponse: Sized {
    /// Wraps `self` in a success `ApiResponse` using HTTP 200.
    ///
    /// # Arguments
    /// * `self` - Value to serialize in the response `data` field.
    /// # Returns
    /// An API response with `success=true` and `StatusCode::OK`.
    fn into_api_ok(self) -> ApiResponse<Self>;
    /// Wraps `self` in a success `ApiResponse` using HTTP 201.
    ///
    /// # Arguments
    /// * `self` - Value to serialize in the response `data` field.
    /// # Returns
    /// An API response with `success=true` and `StatusCode::CREATED`.
    fn into_api_created(self) -> ApiResponse<Self>;
}

impl<T> ApiEnvelope<T, ApiMeta> {
    /// Builds a success envelope with payload `data` and optional metadata.
    ///
    /// # Arguments
    /// * `data` - The payload for the response.
    /// * `meta` - Metadata to include with the response.
    /// # Returns
    /// A populated success envelope.
    pub fn success(data: T, meta: ApiMeta) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            meta,
        }
    }
}

impl ApiEnvelope<(), ApiMeta> {
    /// Builds a success envelope when no response payload is required.
    ///
    /// # Arguments
    /// * `meta` - Metadata to include with the response.
    /// # Returns
    /// A success envelope with `data` set to `None`.
    pub fn empty_success(meta: ApiMeta) -> ApiEnvelope<(), ApiMeta> {
        ApiEnvelope {
            success: true,
            data: None,
            error: None,
            meta,
        }
    }

    /// Builds a failure envelope containing structured API error details.
    ///
    /// # Arguments
    /// * `error` - Error payload to expose under `error`.
    /// * `meta` - Metadata to include with the response.
    /// # Returns
    /// A failure envelope with `success=false` and `data=None`.
    pub fn failure(error: ApiErrorBody, meta: ApiMeta) -> ApiEnvelope<(), ApiMeta> {
        ApiEnvelope {
            success: false,
            data: None,
            error: Some(error),
            meta,
        }
    }
}

impl ApiMeta {
    /// Creates default API metadata with the current timestamp and request duration.
    ///
    /// # Returns
    /// `ApiMeta` with a calculated processing duration and no `details`.
    pub fn new() -> Self {
        let processing_duration = match ApiTimer::current() {
            Some(timer) => timer.elapsed(),
            None => Duration::ZERO,
        };

        Self {
            timestamp: iso_timestamp_now(),
            processing_duration: iso_duration_from_duration(processing_duration),
            details: None,
        }
    }

    /// Creates API metadata with an explicit processing duration override.
    ///
    /// # Arguments
    /// * `processing_duration` - Duration used for the `processing_duration` field.
    /// # Returns
    /// `ApiMeta` with provided timing and no `details`.
    pub fn with_processing_duration(processing_duration: Duration) -> Self {
        Self {
            timestamp: iso_timestamp_now(),
            processing_duration: iso_duration_from_duration(processing_duration),
            details: None,
        }
    }

    /// Attaches arbitrary JSON details to the metadata payload.
    ///
    /// # Arguments
    /// * `self` - Existing metadata.
    /// * `details` - JSON detail object to include.
    /// # Returns
    /// Metadata updated with `details` set.
    pub fn with_details(mut self, details: serde_json::Value) -> Self {
        self.details = Some(details);
        self
    }
}

impl ApiTimer {
    /// Starts a new API timer at the current instant.
    ///
    /// # Returns
    /// A new timer handle suitable for request-scoped timing.
    pub fn start() -> Self {
        Self {
            started_at: Instant::now(),
        }
    }

    /// Returns wall-clock time elapsed since this timer was started.
    ///
    /// # Arguments
    /// * `self` - Timer instance to measure.
    /// # Returns
    /// Duration since `start()`.
    pub fn elapsed(self) -> Duration {
        self.started_at.elapsed()
    }

    pub async fn scope<F>(self, future: F) -> F::Output
    where
        F: Future,
    {
        CURRENT_API_TIMER.scope(self, future).await
    }

    /// Reads the timer currently active in task-local scope.
    ///
    /// # Returns
    /// `Some(ApiTimer)` when a timer is in scope, otherwise `None`.
    fn current() -> Option<Self> {
        CURRENT_API_TIMER.try_with(|timer| *timer).ok()
    }
}

impl Default for ApiMeta {
    /// Delegates to `ApiMeta::new()` for `Default` construction.
    ///
    /// # Returns
    /// A default `ApiMeta` payload.
    fn default() -> Self {
        Self::new()
    }
}

impl ApiErrorBody {
    /// Converts a `CodeError` into an API error body payload.
    ///
    /// # Arguments
    /// * `code_error` - Internal error descriptor.
    /// * `error_detail` - Optional public detail for clients.
    /// # Returns
    /// `ApiErrorBody` populated with mapped HTTP and tracing metadata.
    pub fn from_code_error(code_error: CodeError, error_detail: Option<String>) -> Self {
        Self {
            error_code: code_error.error_code,
            error_level: ApiErrorLevel::from_tracing_level(code_error.log_level),
            error_message: code_error.message,
            error_detail,
        }
    }
}

impl ApiErrorLevel {
    /// Maps a tracing level (`tracing::Level`) into the API error severity enum.
    ///
    /// # Arguments
    /// * `level` - Structured logging level to convert.
    /// # Returns
    /// Matching `ApiErrorLevel` value.
    pub fn from_tracing_level(level: tracing::Level) -> Self {
        if level == tracing::Level::ERROR {
            return Self::Error;
        }

        if level == tracing::Level::WARN {
            return Self::Warn;
        }

        if level == tracing::Level::DEBUG {
            return Self::Debug;
        }

        if level == tracing::Level::TRACE {
            return Self::Trace;
        }

        Self::Info
    }
}

impl<T> ApiResponse<T> {
    /// Builds a success response with HTTP 200 and payload data.
    ///
    /// # Arguments
    /// * `data` - Response payload.
    /// # Returns
    /// API response with `status_code = 200`.
    pub fn ok(data: T) -> Self {
        Self::success(StatusCode::OK, data)
    }

    /// Builds a success response with HTTP 201 and payload data.
    ///
    /// # Arguments
    /// * `data` - Response payload.
    /// # Returns
    /// API response with `status_code = 201`.
    pub fn created(data: T) -> Self {
        Self::success(StatusCode::CREATED, data)
    }

    /// Builds a success response for an arbitrary status code.
    ///
    /// # Arguments
    /// * `status_code` - HTTP status for this response.
    /// * `data` - Response payload.
    /// # Returns
    /// API response using fresh `ApiMeta` and `success=true` envelope.
    pub fn success(status_code: StatusCode, data: T) -> Self {
        Self {
            status_code,
            body: ApiEnvelope::success(data, ApiMeta::new()),
        }
    }
}

impl ApiResponse<()> {
    /// Returns an empty success response with HTTP 200.
    ///
    /// # Returns
    /// API response with no payload.
    pub fn empty() -> Self {
        Self::empty_with_status(StatusCode::OK)
    }

    /// Returns an empty success response with a caller-chosen status.
    ///
    /// # Arguments
    /// * `status_code` - HTTP status for the response.
    /// # Returns
    /// Response envelope marked successful with `data=None`.
    pub fn empty_with_status(status_code: StatusCode) -> Self {
        Self {
            status_code,
            body: ApiEnvelope::<(), ApiMeta>::empty_success(ApiMeta::new()),
        }
    }
}

impl<T> ApiResponse<T> {
    /// Adds debug metadata details to an existing response.
    ///
    /// # Arguments
    /// * `self` - Existing response.
    /// * `details` - JSON body to include under `meta.details`.
    /// # Returns
    /// A cloned response with metadata enriched.
    pub fn with_meta_details(mut self, details: serde_json::Value) -> Self {
        self.body.meta = self.body.meta.with_details(details);
        self
    }

    /// Returns the HTTP status code configured for this response.
    ///
    /// # Arguments
    /// * `self` - Response instance.
    /// # Returns
    /// The response status.
    pub fn status_code(&self) -> StatusCode {
        self.status_code
    }

    /// Moves the response out of the wrapper and returns the envelope.
    ///
    /// # Arguments
    /// * `self` - Response instance.
    /// # Returns
    /// The envelope body that Axum converts to JSON.
    pub fn into_body(self) -> ApiEnvelope<T, ApiMeta> {
        self.body
    }
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    /// Converts the response into an Axum HTTP response.
    ///
    /// # Arguments
    /// * `self` - Response wrapper containing status and body.
    /// # Returns
    /// `(status, Json(body))` as an Axum response.
    fn into_response(self) -> Response {
        (self.status_code, Json(self.body)).into_response()
    }
}

impl<T> IntoApiResponse for T {
    /// Converts a value into an `ApiResponse` with HTTP 200.
    ///
    /// # Arguments
    /// * `self` -
    /// # Returns
    /// Returns the value produced by this function.
    fn into_api_ok(self) -> ApiResponse<Self> {
        ApiResponse::ok(self)
    }

    /// Converts a value into an `ApiResponse` with HTTP 201.
    ///
    /// # Arguments
    /// * `self` -
    /// # Returns
    /// Returns the value produced by this function.
    fn into_api_created(self) -> ApiResponse<Self> {
        ApiResponse::created(self)
    }
}

pub fn api_ok<T>(data: T) -> ApiResponse<T> {
    ApiResponse::ok(data)
}

pub fn api_created<T>(data: T) -> ApiResponse<T> {
    ApiResponse::created(data)
}

/// Returns an empty API response (`status_code = 200`).
///
/// # Arguments
/// * `) -> ApiResponse<(` -
/// # Returns
/// Returns the value produced by this function.
pub fn api_empty() -> ApiResponse<()> {
    ApiResponse::empty()
}

/// Returns the current UTC timestamp in RFC3339 milliseconds format.
///
/// # Returns
/// Returns the value produced by this function.
fn iso_timestamp_now() -> String {
    Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true)
}

/// Converts a `Duration` into ISO 8601 duration syntax used by responses.
///
/// # Arguments
/// * `duration` - Elapsed duration to serialize.
/// # Returns
/// Returns the value produced by this function.
pub fn iso_duration_from_duration(duration: Duration) -> String {
    let seconds = duration.as_secs();
    let nanos = duration.subsec_nanos();

    if nanos == 0 {
        return format!("PT{seconds}S");
    }

    let fractional_seconds = format!("{nanos:09}");
    let trimmed_fractional_seconds = fractional_seconds.trim_end_matches('0');

    format!("PT{seconds}.{trimmed_fractional_seconds}S")
}
