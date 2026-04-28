use std::fmt;

use axum::http::StatusCode;
use tracing::Level;

#[derive(Copy, Clone, Debug)]
pub struct CodeError {
    pub success: bool,
    pub error_code: u8,
    pub http_status_code: StatusCode,
    pub message: &'static str,
    pub log_level: Level,
}

impl CodeError {
    pub const INTERNAL_ERROR: CodeError = CodeError {
        success: false,
        error_code: 255,
        http_status_code: StatusCode::INTERNAL_SERVER_ERROR,
        message: "Internal server error!",
        log_level: Level::ERROR,
    };
}

impl fmt::Display for CodeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.message)
    }
}
