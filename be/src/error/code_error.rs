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
    pub const DB_POOL_ERROR: CodeError = CodeError {
        success: false,
        error_code: 1,
        http_status_code: StatusCode::INTERNAL_SERVER_ERROR,
        message: "Could not get database connection from pool!",
        log_level: Level::ERROR,
    };
    pub const DB_QUERY_ERROR: CodeError = CodeError {
        success: false,
        error_code: 2,
        http_status_code: StatusCode::INTERNAL_SERVER_ERROR,
        message: "Database query failed!",
        log_level: Level::ERROR,
    };
    pub const DB_INSERT_ERROR: CodeError = CodeError {
        success: false,
        error_code: 3,
        http_status_code: StatusCode::INTERNAL_SERVER_ERROR,
        message: "Database insert failed!",
        log_level: Level::ERROR,
    };
    pub const DB_UPDATE_ERROR: CodeError = CodeError {
        success: false,
        error_code: 4,
        http_status_code: StatusCode::INTERNAL_SERVER_ERROR,
        message: "Database update failed!",
        log_level: Level::ERROR,
    };
    pub const EMAIL_INVALID: CodeError = CodeError {
        success: false,
        error_code: 5,
        http_status_code: StatusCode::BAD_REQUEST,
        message: "Invalid email address!",
        log_level: Level::INFO,
    };
    pub const USER_NAME_INVALID: CodeError = CodeError {
        success: false,
        error_code: 6,
        http_status_code: StatusCode::BAD_REQUEST,
        message: "Invalid username!",
        log_level: Level::INFO,
    };
    pub const PASSWORD_INVALID: CodeError = CodeError {
        success: false,
        error_code: 7,
        http_status_code: StatusCode::BAD_REQUEST,
        message: "Invalid password!",
        log_level: Level::INFO,
    };
    pub const EMAIL_ALREADY_EXISTS: CodeError = CodeError {
        success: false,
        error_code: 8,
        http_status_code: StatusCode::BAD_REQUEST,
        message: "Email address is already registered!",
        log_level: Level::INFO,
    };
    pub const USER_NOT_FOUND: CodeError = CodeError {
        success: false,
        error_code: 9,
        http_status_code: StatusCode::NOT_FOUND,
        message: "User not found!",
        log_level: Level::INFO,
    };
    pub const WRONG_PASSWORD: CodeError = CodeError {
        success: false,
        error_code: 10,
        http_status_code: StatusCode::UNAUTHORIZED,
        message: "Wrong password!",
        log_level: Level::INFO,
    };
    pub const PASSWORD_HASH_ERROR: CodeError = CodeError {
        success: false,
        error_code: 11,
        http_status_code: StatusCode::INTERNAL_SERVER_ERROR,
        message: "Failed to hash password!",
        log_level: Level::ERROR,
    };
    pub const PASSWORD_VERIFY_ERROR: CodeError = CodeError {
        success: false,
        error_code: 12,
        http_status_code: StatusCode::INTERNAL_SERVER_ERROR,
        message: "Failed to verify password!",
        log_level: Level::ERROR,
    };
    pub const UNAUTHORIZED: CodeError = CodeError {
        success: false,
        error_code: 13,
        http_status_code: StatusCode::UNAUTHORIZED,
        message: "Unauthorized!",
        log_level: Level::INFO,
    };
    pub const JWT_INVALID: CodeError = CodeError {
        success: false,
        error_code: 14,
        http_status_code: StatusCode::UNAUTHORIZED,
        message: "Invalid access token!",
        log_level: Level::INFO,
    };
    pub const EMAIL_NOT_VERIFIED: CodeError = CodeError {
        success: false,
        error_code: 15,
        http_status_code: StatusCode::FORBIDDEN,
        message: "Email address is not verified!",
        log_level: Level::INFO,
    };
    pub const EMAIL_VERIFICATION_TOKEN_INVALID: CodeError = CodeError {
        success: false,
        error_code: 16,
        http_status_code: StatusCode::BAD_REQUEST,
        message: "Invalid email verification token!",
        log_level: Level::INFO,
    };
    pub const EMAIL_VERIFICATION_TOKEN_EXPIRED: CodeError = CodeError {
        success: false,
        error_code: 17,
        http_status_code: StatusCode::BAD_REQUEST,
        message: "Email verification token expired!",
        log_level: Level::INFO,
    };
    pub const EMAIL_VERIFICATION_TOKEN_ALREADY_USED: CodeError = CodeError {
        success: false,
        error_code: 18,
        http_status_code: StatusCode::BAD_REQUEST,
        message: "Email verification token already used!",
        log_level: Level::INFO,
    };
    pub const PASSWORD_RESET_TOKEN_INVALID: CodeError = CodeError {
        success: false,
        error_code: 19,
        http_status_code: StatusCode::BAD_REQUEST,
        message: "Invalid password reset token!",
        log_level: Level::INFO,
    };
    pub const PASSWORD_RESET_TOKEN_EXPIRED: CodeError = CodeError {
        success: false,
        error_code: 20,
        http_status_code: StatusCode::BAD_REQUEST,
        message: "Password reset token expired!",
        log_level: Level::INFO,
    };
    pub const PASSWORD_RESET_TOKEN_ALREADY_USED: CodeError = CodeError {
        success: false,
        error_code: 21,
        http_status_code: StatusCode::BAD_REQUEST,
        message: "Password reset token already used!",
        log_level: Level::INFO,
    };
    pub const DATABASE_UNSUPPORTED: CodeError = CodeError {
        success: false,
        error_code: 22,
        http_status_code: StatusCode::INTERNAL_SERVER_ERROR,
        message: "Configured database backend is not implemented!",
        log_level: Level::ERROR,
    };
    pub const RATE_LIMITED: CodeError = CodeError {
        success: false,
        error_code: 23,
        http_status_code: StatusCode::TOO_MANY_REQUESTS,
        message: "Too many requests!",
        log_level: Level::WARN,
    };
    pub const REFERENCE_DATA_NOT_FOUND: CodeError = CodeError {
        success: false,
        error_code: 24,
        http_status_code: StatusCode::NOT_FOUND,
        message: "Reference data not found!",
        log_level: Level::INFO,
    };
    pub const EMAIL_VERIFICATION_CHALLENGE_INVALID: CodeError = CodeError {
        success: false,
        error_code: 25,
        http_status_code: StatusCode::BAD_REQUEST,
        message: "Invalid email verification challenge!",
        log_level: Level::INFO,
    };
    pub const EMAIL_VERIFICATION_CHALLENGE_EXPIRED: CodeError = CodeError {
        success: false,
        error_code: 26,
        http_status_code: StatusCode::BAD_REQUEST,
        message: "Email verification challenge expired!",
        log_level: Level::INFO,
    };
    pub const EMAIL_VERIFICATION_CHALLENGE_FAILED: CodeError = CodeError {
        success: false,
        error_code: 27,
        http_status_code: StatusCode::BAD_REQUEST,
        message: "Email verification challenge failed!",
        log_level: Level::INFO,
    };
    pub const EMAIL_VERIFICATION_ANSWER_INVALID: CodeError = CodeError {
        success: false,
        error_code: 28,
        http_status_code: StatusCode::BAD_REQUEST,
        message: "Email verification answer is incorrect!",
        log_level: Level::INFO,
    };
    pub const ADMIN_REQUIRED: CodeError = CodeError {
        success: false,
        error_code: 29,
        http_status_code: StatusCode::FORBIDDEN,
        message: "Admin access required!",
        log_level: Level::INFO,
    };
    pub const VALIDATION_FAILED: CodeError = CodeError {
        success: false,
        error_code: 30,
        http_status_code: StatusCode::BAD_REQUEST,
        message: "Validation failed!",
        log_level: Level::INFO,
    };
    pub const ROUTE_NOT_FOUND: CodeError = CodeError {
        success: false,
        error_code: 31,
        http_status_code: StatusCode::NOT_FOUND,
        message: "Route not found!",
        log_level: Level::INFO,
    };
    pub const REFRESH_SESSION_INVALID: CodeError = CodeError {
        success: false,
        error_code: 32,
        http_status_code: StatusCode::UNAUTHORIZED,
        message: "Invalid refresh session!",
        log_level: Level::INFO,
    };
    pub const REFRESH_SESSION_EXPIRED: CodeError = CodeError {
        success: false,
        error_code: 33,
        http_status_code: StatusCode::UNAUTHORIZED,
        message: "Refresh session expired!",
        log_level: Level::INFO,
    };
    pub const ORIGIN_NOT_ALLOWED: CodeError = CodeError {
        success: false,
        error_code: 34,
        http_status_code: StatusCode::FORBIDDEN,
        message: "Request origin is not allowed!",
        log_level: Level::WARN,
    };
    pub const SERVICE_PROVIDER_REQUIRED: CodeError = CodeError {
        success: false,
        error_code: 35,
        http_status_code: StatusCode::FORBIDDEN,
        message: "Service provider access required!",
        log_level: Level::INFO,
    };
    pub const MODERATOR_REQUIRED: CodeError = CodeError {
        success: false,
        error_code: 36,
        http_status_code: StatusCode::FORBIDDEN,
        message: "Moderator access required!",
        log_level: Level::INFO,
    };
    pub const MARKETPLACE_NOT_FOUND: CodeError = CodeError {
        success: false,
        error_code: 37,
        http_status_code: StatusCode::NOT_FOUND,
        message: "Marketplace resource not found!",
        log_level: Level::INFO,
    };
    pub const INTERNAL_ERROR: CodeError = CodeError {
        success: false,
        error_code: 255,
        http_status_code: StatusCode::INTERNAL_SERVER_ERROR,
        message: "Internal server error!",
        log_level: Level::ERROR,
    };
}

impl fmt::Display for CodeError {
    /// Formats a `CodeError` as its public message for Display output.
    ///
    /// # Arguments
    /// * `self` -
    /// * `formatter` -
    /// # Returns
    /// Returns the value produced by this function.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.message)
    }
}
