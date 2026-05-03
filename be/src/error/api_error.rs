use std::fmt;

use axum::{
    Json,
    response::{IntoResponse, Response},
};
use tracing::{Level, error, info, warn};

use crate::{
    dto::api_response::{ApiEnvelope, ApiErrorBody, ApiMeta, ApiResult},
    error::code_error::CodeError,
};

#[derive(Debug, Clone)]
pub struct ApiError {
    code_error: CodeError,
    source_error: Option<String>,
    public_detail: Option<String>,
}

pub trait ApiResultExt<T, E> {
    /// Converts a `Result<T, E>` into an API result by mapping `Err` into an `ApiError`.
    ///
    /// # Arguments
    /// * `self` - A `Result` from lower-level code paths.
    /// * `code_error` - Default API error to emit on `Err`.
    /// # Returns
    /// A `Result`, either containing the function output or an error.
    fn api_err(self, code_error: CodeError) -> ApiResult<T>
    where
        E: fmt::Display;

    fn api_err_public<D>(self, code_error: CodeError, public_detail: D) -> ApiResult<T>
    where
        E: fmt::Display,
        D: Into<String>;

    fn api_err_with_detail<D>(self, code_error: CodeError, detail: D) -> ApiResult<T>
    where
        E: fmt::Display,
        D: FnOnce(&E) -> String;
}

pub trait ApiOptionExt<T> {
    /// Converts `Option`/`Result` helper calls into API results with a configured error code.
    ///
    /// # Arguments
    /// * `self` - A value or `None` from domain access.
    /// * `code_error` - Default API error when value is absent.
    /// # Returns
    /// A `Result`, either containing the function output or an error.
    fn api_ok_or(self, code_error: CodeError) -> ApiResult<T>;

    fn api_ok_or_public<D>(self, code_error: CodeError, public_detail: D) -> ApiResult<T>
    where
        D: Into<String>;
}

impl ApiError {
    /// Creates a fresh `ApiError` from a structured `CodeError` without source detail.
    ///
    /// # Arguments
    /// * `code_error` -
    /// # Returns
    /// A new `ApiError` instance.
    pub fn new(code_error: CodeError) -> Self {
        Self {
            code_error,
            source_error: None,
            public_detail: None,
        }
    }

    pub fn public<D>(code_error: CodeError, public_detail: D) -> Self
    where
        D: Into<String>,
    {
        Self {
            code_error,
            source_error: None,
            public_detail: Some(public_detail.into()),
        }
    }

    pub fn from_source<E>(code_error: CodeError, source_error: E) -> Self
    where
        E: fmt::Display,
    {
        Self {
            code_error,
            source_error: Some(source_error.to_string()),
            public_detail: None,
        }
    }

    pub fn from_source_public<E, D>(
        code_error: CodeError,
        source_error: E,
        public_detail: D,
    ) -> Self
    where
        E: fmt::Display,
        D: Into<String>,
    {
        Self {
            code_error,
            source_error: Some(source_error.to_string()),
            public_detail: Some(public_detail.into()),
        }
    }

    pub fn with_public_detail<D>(mut self, public_detail: D) -> Self
    where
        D: Into<String>,
    {
        self.public_detail = Some(public_detail.into());
        self
    }

    /// Returns the embedded `CodeError` value for this API error.
    ///
    /// # Arguments
    /// * `self` -
    /// # Returns
    /// The stored `CodeError` metadata.
    pub fn code_error(&self) -> CodeError {
        self.code_error
    }

    /// Logs the error using structured fields and the configured severity.
    ///
    /// # Arguments
    /// * `self` -
    /// # Returns
    /// No value is returned.
    pub fn log(&self) {
        match self.code_error.log_level {
            Level::ERROR => {
                error!(
                    error_code = self.code_error.error_code,
                    http_status_code = self.code_error.http_status_code.as_u16(),
                    error_message = self.code_error.message,
                    error_detail = ?self.public_detail,
                    source_error = ?self.source_error,
                    "API request failed"
                );
            }
            Level::WARN => {
                warn!(
                    error_code = self.code_error.error_code,
                    http_status_code = self.code_error.http_status_code.as_u16(),
                    error_message = self.code_error.message,
                    error_detail = ?self.public_detail,
                    source_error = ?self.source_error,
                    "API request failed"
                );
            }
            Level::INFO | Level::DEBUG | Level::TRACE => {
                info!(
                    error_code = self.code_error.error_code,
                    http_status_code = self.code_error.http_status_code.as_u16(),
                    error_message = self.code_error.message,
                    error_detail = ?self.public_detail,
                    source_error = ?self.source_error,
                    "API request failed"
                );
            }
        }
    }

    /// Builds API response metadata for error responses using current timing context.
    ///
    /// # Arguments
    /// * `self` -
    /// # Returns
    /// A populated `ApiMeta` struct for the error response.
    fn response_meta(&self) -> ApiMeta {
        ApiMeta::new()
    }

    /// Builds the standardized failed-envelope body (`success=false`) for HTTP responses.
    ///
    /// # Arguments
    /// * `self` -
    /// # Returns
    /// A standardized failed response envelope.
    fn response_body(&self) -> ApiEnvelope<(), ApiMeta> {
        ApiEnvelope::failure(
            ApiErrorBody::from_code_error(self.code_error, self.public_detail.clone()),
            self.response_meta(),
        )
    }
}

impl<T, E> ApiResultExt<T, E> for Result<T, E> {
    /// Converts a `Result<T, E>` into an API result by mapping `Err` into an `ApiError`.
    ///
    /// # Arguments
    /// * `self` -
    /// * `code_error` -
    /// # Returns
    /// A `Result`, either containing the function output or an error.
    fn api_err(self, code_error: CodeError) -> ApiResult<T>
    where
        E: fmt::Display,
    {
        match self {
            Ok(value) => Ok(value),
            Err(error) => Err(ApiError::from_source(code_error, error)),
        }
    }

    fn api_err_public<D>(self, code_error: CodeError, public_detail: D) -> ApiResult<T>
    where
        E: fmt::Display,
        D: Into<String>,
    {
        match self {
            Ok(value) => Ok(value),
            Err(error) => Err(ApiError::from_source_public(
                code_error,
                error,
                public_detail,
            )),
        }
    }

    fn api_err_with_detail<D>(self, code_error: CodeError, detail: D) -> ApiResult<T>
    where
        E: fmt::Display,
        D: FnOnce(&E) -> String,
    {
        match self {
            Ok(value) => Ok(value),
            Err(error) => {
                let public_detail = detail(&error);
                Err(ApiError::from_source_public(
                    code_error,
                    error,
                    public_detail,
                ))
            }
        }
    }
}

impl<T> ApiOptionExt<T> for Option<T> {
    /// Converts `Option`/`Result` helper calls into API results with a configured error code.
    ///
    /// # Arguments
    /// * `self` - Optional source value.
    /// * `code_error` - API code to use when optional value is absent.
    /// # Returns
    /// A `Result`, either containing the function output or an error.
    fn api_ok_or(self, code_error: CodeError) -> ApiResult<T> {
        match self {
            Some(value) => Ok(value),
            None => Err(ApiError::new(code_error)),
        }
    }

    fn api_ok_or_public<D>(self, code_error: CodeError, public_detail: D) -> ApiResult<T>
    where
        D: Into<String>,
    {
        match self {
            Some(value) => Ok(value),
            None => Err(ApiError::public(code_error, public_detail)),
        }
    }
}

impl From<CodeError> for ApiError {
    /// Converts the source type into an API error representation.
    ///
    /// # Arguments
    /// * `code_error` -
    /// # Returns
    /// An `ApiError` seeded with the provided `CodeError`.
    fn from(code_error: CodeError) -> Self {
        Self::new(code_error)
    }
}

impl From<diesel::result::Error> for ApiError {
    /// Converts the source type into an API error representation.
    ///
    /// # Arguments
    /// * `error` -
    /// # Returns
    /// An `ApiError` with `CodeError::DB_QUERY_ERROR`.
    fn from(error: diesel::result::Error) -> Self {
        Self::from_source(CodeError::DB_QUERY_ERROR, error)
    }
}

impl IntoResponse for CodeError {
    /// Converts this error value into an HTTP response.
    ///
    /// # Arguments
    /// * `self` -
    /// # Returns
    /// An Axum response with status/body derived from this code error.
    fn into_response(self) -> Response {
        ApiError::new(self).into_response()
    }
}

impl fmt::Display for ApiError {
    /// Formats the public message and appends source detail when present.
    ///
    /// # Arguments
    /// * `self` -
    /// * `formatter` -
    /// # Returns
    /// Result of writing the display representation.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.source_error {
            Some(source_error) => {
                write!(formatter, "{}: {source_error}", self.code_error.message)
            }
            None => formatter.write_str(self.code_error.message),
        }
    }
}

impl std::error::Error for ApiError {}

impl IntoResponse for ApiError {
    /// Converts this error value into an HTTP response.
    ///
    /// # Arguments
    /// * `self` -
    /// # Returns
    /// An Axum response with configured status and body.
    fn into_response(self) -> Response {
        self.log();
        (self.code_error.http_status_code, Json(self.response_body())).into_response()
    }
}
