use crate::{
    dto::api_response::ApiResult,
    error::{api_error::ApiError, code_error::CodeError},
};

const MAX_SHORT_TEXT: usize = 180;
const MAX_LONG_TEXT: usize = 12000;
const MAX_SLUG: usize = 80;

pub fn required_text(value: String, max_len: usize, field: &str) -> ApiResult<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() || trimmed.chars().count() > max_len {
        return Err(ApiError::public(
            CodeError::VALIDATION_FAILED,
            format!("{field} is required and must be at most {max_len} characters"),
        ));
    }

    Ok(trimmed.to_string())
}

pub fn optional_text(
    value: Option<String>,
    max_len: usize,
    field: &str,
) -> ApiResult<Option<String>> {
    match value {
        Some(value) => {
            let trimmed = value.trim();
            if trimmed.is_empty() {
                return Ok(None);
            }
            if trimmed.chars().count() > max_len {
                return Err(ApiError::public(
                    CodeError::VALIDATION_FAILED,
                    format!("{field} must be at most {max_len} characters"),
                ));
            }
            Ok(Some(trimmed.to_string()))
        }
        None => Ok(None),
    }
}

pub fn short_optional(value: Option<String>, field: &str) -> ApiResult<Option<String>> {
    optional_text(value, MAX_SHORT_TEXT, field)
}

pub fn long_optional(value: Option<String>, field: &str) -> ApiResult<Option<String>> {
    optional_text(value, MAX_LONG_TEXT, field)
}

pub fn required_short(value: String, field: &str) -> ApiResult<String> {
    required_text(value, MAX_SHORT_TEXT, field)
}

pub fn required_long(value: String, field: &str) -> ApiResult<String> {
    required_text(value, MAX_LONG_TEXT, field)
}

pub fn slug_from(input: Option<String>, fallback: &str) -> ApiResult<String> {
    let source = match input {
        Some(value) => value,
        None => fallback.to_string(),
    };
    let mut slug = String::with_capacity(source.len());
    let mut previous_dash = false;

    for character in source.chars() {
        if character.is_ascii_alphanumeric() {
            slug.push(character.to_ascii_lowercase());
            previous_dash = false;
        } else if (character.is_ascii_whitespace() || character == '-' || character == '_')
            && !previous_dash
            && !slug.is_empty()
        {
            slug.push('-');
            previous_dash = true;
        }
    }

    let trimmed = slug.trim_matches('-').to_string();
    let len = trimmed.chars().count();
    if !(3..=MAX_SLUG).contains(&len) {
        return Err(ApiError::public(
            CodeError::VALIDATION_FAILED,
            "slug must contain 3 to 80 ASCII letters, digits, or hyphen separators",
        ));
    }

    Ok(trimmed)
}

pub fn directory_limit(limit: Option<i64>) -> i64 {
    match limit {
        Some(value) if (1..=100).contains(&value) => value,
        Some(_) | None => 24,
    }
}

pub fn validate_image_metadata(
    bucket: &str,
    object_key: &str,
    mime_type: &str,
    byte_size: i64,
    width: Option<i32>,
    height: Option<i32>,
) -> ApiResult<()> {
    if bucket.trim().is_empty() || object_key.trim().is_empty() {
        return Err(ApiError::public(
            CodeError::VALIDATION_FAILED,
            "image bucket and object key are required",
        ));
    }

    match mime_type {
        "image/jpeg" | "image/png" | "image/webp" | "image/gif" => {}
        _ => {
            return Err(ApiError::public(
                CodeError::VALIDATION_FAILED,
                "image mime type must be jpeg, png, webp, or gif",
            ));
        }
    }

    if byte_size <= 0 || byte_size > 20_971_520 {
        return Err(ApiError::public(
            CodeError::VALIDATION_FAILED,
            "image byte size must be between 1 byte and 20 MiB",
        ));
    }

    match width {
        Some(value) if value <= 0 => {
            return Err(ApiError::public(
                CodeError::VALIDATION_FAILED,
                "image width must be positive",
            ));
        }
        Some(_) | None => {}
    }

    match height {
        Some(value) if value <= 0 => {
            return Err(ApiError::public(
                CodeError::VALIDATION_FAILED,
                "image height must be positive",
            ));
        }
        Some(_) | None => {}
    }

    Ok(())
}
