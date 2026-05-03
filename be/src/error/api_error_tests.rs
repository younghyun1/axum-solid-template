use axum::response::IntoResponse;

use crate::error::{
    api_error::{ApiOptionExt, ApiResultExt},
    code_error::CodeError,
};

#[test]
fn result_extension_maps_library_error_to_api_error() {
    let result: Result<(), std::io::Error> = Err(std::io::Error::other("disk unavailable"));
    let mapped = result.api_err(CodeError::INTERNAL_ERROR);
    assert!(mapped.is_err());

    let api_error = match mapped {
        Ok(()) => return,
        Err(api_error) => api_error,
    };

    assert_eq!(api_error.code_error().error_code, 255);
}

#[test]
fn option_extension_maps_none_to_api_error() {
    let missing: Option<u8> = None;
    let mapped = missing.api_ok_or(CodeError::INTERNAL_ERROR);
    assert!(mapped.is_err());

    let api_error = match mapped {
        Ok(_) => return,
        Err(api_error) => api_error,
    };

    assert_eq!(
        api_error.code_error().http_status_code,
        CodeError::INTERNAL_ERROR.http_status_code
    );
}

#[test]
fn api_error_into_response_uses_configured_status_code() {
    let response = CodeError::INTERNAL_ERROR.into_response();
    assert_eq!(
        response.status(),
        CodeError::INTERNAL_ERROR.http_status_code
    );
}
