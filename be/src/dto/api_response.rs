use std::time::{Duration, Instant};

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
    fn into_api_ok(self) -> ApiResponse<Self>;
    fn into_api_created(self) -> ApiResponse<Self>;
}

impl<T> ApiEnvelope<T, ApiMeta> {
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
    pub fn empty_success(meta: ApiMeta) -> ApiEnvelope<(), ApiMeta> {
        ApiEnvelope {
            success: true,
            data: None,
            error: None,
            meta,
        }
    }

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
    pub fn new() -> Self {
        Self {
            timestamp: iso_timestamp_now(),
            processing_duration: iso_duration_from_duration(Duration::ZERO),
            details: None,
        }
    }

    pub fn from_timer(timer: ApiTimer) -> Self {
        Self {
            timestamp: iso_timestamp_now(),
            processing_duration: iso_duration_from_duration(timer.elapsed()),
            details: None,
        }
    }

    pub fn with_processing_duration(processing_duration: Duration) -> Self {
        Self {
            timestamp: iso_timestamp_now(),
            processing_duration: iso_duration_from_duration(processing_duration),
            details: None,
        }
    }

    pub fn with_details(mut self, details: serde_json::Value) -> Self {
        self.details = Some(details);
        self
    }

    pub fn with_timer(mut self, timer: ApiTimer) -> Self {
        self.processing_duration = iso_duration_from_duration(timer.elapsed());
        self
    }
}

impl Default for ApiMeta {
    fn default() -> Self {
        Self {
            timestamp: iso_timestamp_now(),
            processing_duration: iso_duration_from_duration(Duration::ZERO),
            details: None,
        }
    }
}

impl ApiTimer {
    pub fn start() -> Self {
        Self {
            started_at: Instant::now(),
        }
    }

    pub fn elapsed(self) -> Duration {
        self.started_at.elapsed()
    }
}

impl ApiErrorBody {
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
    pub fn ok(data: T) -> Self {
        Self::success(StatusCode::OK, data)
    }

    pub fn ok_timed(data: T, timer: ApiTimer) -> Self {
        Self::success_timed(StatusCode::OK, data, timer)
    }

    pub fn created(data: T) -> Self {
        Self::success(StatusCode::CREATED, data)
    }

    pub fn created_timed(data: T, timer: ApiTimer) -> Self {
        Self::success_timed(StatusCode::CREATED, data, timer)
    }

    pub fn success(status_code: StatusCode, data: T) -> Self {
        Self {
            status_code,
            body: ApiEnvelope::success(data, ApiMeta::new()),
        }
    }

    pub fn success_timed(status_code: StatusCode, data: T, timer: ApiTimer) -> Self {
        Self {
            status_code,
            body: ApiEnvelope::success(data, ApiMeta::from_timer(timer)),
        }
    }
}

impl ApiResponse<()> {
    pub fn empty() -> Self {
        Self::empty_with_status(StatusCode::OK)
    }

    pub fn empty_timed(timer: ApiTimer) -> Self {
        Self::empty_with_status_timed(StatusCode::OK, timer)
    }

    pub fn empty_with_status(status_code: StatusCode) -> Self {
        Self {
            status_code,
            body: ApiEnvelope::<(), ApiMeta>::empty_success(ApiMeta::new()),
        }
    }

    pub fn empty_with_status_timed(status_code: StatusCode, timer: ApiTimer) -> Self {
        Self {
            status_code,
            body: ApiEnvelope::<(), ApiMeta>::empty_success(ApiMeta::from_timer(timer)),
        }
    }
}

impl<T> ApiResponse<T> {
    pub fn with_timer(mut self, timer: ApiTimer) -> Self {
        self.body.meta = self.body.meta.with_timer(timer);
        self
    }

    pub fn with_meta_details(mut self, details: serde_json::Value) -> Self {
        self.body.meta = self.body.meta.with_details(details);
        self
    }

    pub fn status_code(&self) -> StatusCode {
        self.status_code
    }

    pub fn into_body(self) -> ApiEnvelope<T, ApiMeta> {
        self.body
    }
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        (self.status_code, Json(self.body)).into_response()
    }
}

impl<T> IntoApiResponse for T {
    fn into_api_ok(self) -> ApiResponse<Self> {
        ApiResponse::ok(self)
    }

    fn into_api_created(self) -> ApiResponse<Self> {
        ApiResponse::created(self)
    }
}

pub fn api_ok<T>(data: T) -> ApiResponse<T> {
    ApiResponse::ok(data)
}

pub fn api_ok_timed<T>(data: T, timer: ApiTimer) -> ApiResponse<T> {
    ApiResponse::ok_timed(data, timer)
}

pub fn api_created<T>(data: T) -> ApiResponse<T> {
    ApiResponse::created(data)
}

pub fn api_created_timed<T>(data: T, timer: ApiTimer) -> ApiResponse<T> {
    ApiResponse::created_timed(data, timer)
}

pub fn api_empty() -> ApiResponse<()> {
    ApiResponse::empty()
}

pub fn api_empty_timed(timer: ApiTimer) -> ApiResponse<()> {
    ApiResponse::empty_timed(timer)
}

fn iso_timestamp_now() -> String {
    Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true)
}

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

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use serde::Serialize;
    use serde_json::{Value, json};

    use super::{ApiMeta, ApiResponse};

    #[derive(Debug, Serialize)]
    struct TestData {
        value: u8,
    }

    #[test]
    fn success_response_serializes_with_standard_shape() {
        let response = ApiResponse::ok(TestData { value: 7 });
        let serialized_result = serde_json::to_value(response.into_body());
        assert!(serialized_result.is_ok());

        let serialized = match serialized_result {
            Ok(value) => value,
            Err(_) => return,
        };

        assert_eq!(serialized["success"], json!(true));
        assert_eq!(serialized["data"]["value"], json!(7));
        assert_eq!(serialized["error"], Value::Null);
        assert!(serialized["meta"]["timestamp"].is_string());
        assert_eq!(serialized["meta"]["processing_duration"], json!("PT0S"));
    }

    #[test]
    fn meta_details_are_optional() {
        let meta = ApiMeta::with_processing_duration(Duration::from_micros(1042))
            .with_details(json!({"page": 1}));
        let serialized_result = serde_json::to_value(meta);
        assert!(serialized_result.is_ok());

        let serialized = match serialized_result {
            Ok(value) => value,
            Err(_) => return,
        };

        assert_eq!(serialized["processing_duration"], json!("PT0.001042S"));
        assert_eq!(serialized["details"]["page"], json!(1));
    }
}
