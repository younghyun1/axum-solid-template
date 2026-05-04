use std::{fmt, time::Duration};

use axum::http::{
    HeaderMap, HeaderValue,
    header::{COOKIE, SET_COOKIE},
};

use crate::init::server_config::http_security_config::CookieConfig;

pub const ACCESS_TOKEN_COOKIE_NAME: &str = "access_token";
pub const REFRESH_SESSION_COOKIE_NAME: &str = "refresh_session";
pub const ACCESS_TOKEN_COOKIE_PATH: &str = "/api/v1";
pub const REFRESH_SESSION_COOKIE_PATH: &str = "/api/v1/auth";

#[derive(Debug)]
pub struct CookieHeaderError {
    detail: String,
}

impl fmt::Display for CookieHeaderError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.detail)
    }
}

pub fn access_token_from_headers(headers: &HeaderMap) -> Option<String> {
    cookie_value(headers, ACCESS_TOKEN_COOKIE_NAME)
}

pub fn refresh_session_from_headers(headers: &HeaderMap) -> Option<String> {
    cookie_value(headers, REFRESH_SESSION_COOKIE_NAME)
}

pub fn auth_cookie_headers(
    cookie_config: &CookieConfig,
    access_token: &str,
    access_max_age: Duration,
    refresh_token: &str,
    refresh_max_age: Duration,
) -> Result<Vec<HeaderValue>, CookieHeaderError> {
    let access_header = match set_cookie_header(
        cookie_config,
        ACCESS_TOKEN_COOKIE_NAME,
        access_token,
        ACCESS_TOKEN_COOKIE_PATH,
        access_max_age.as_secs(),
    ) {
        Ok(header) => header,
        Err(error) => return Err(error),
    };
    let refresh_header = match set_cookie_header(
        cookie_config,
        REFRESH_SESSION_COOKIE_NAME,
        refresh_token,
        REFRESH_SESSION_COOKIE_PATH,
        refresh_max_age.as_secs(),
    ) {
        Ok(header) => header,
        Err(error) => return Err(error),
    };

    Ok(vec![access_header, refresh_header])
}

pub fn clear_auth_cookie_headers(
    cookie_config: &CookieConfig,
) -> Result<Vec<HeaderValue>, CookieHeaderError> {
    let access_header = match set_cookie_header(
        cookie_config,
        ACCESS_TOKEN_COOKIE_NAME,
        "",
        ACCESS_TOKEN_COOKIE_PATH,
        0,
    ) {
        Ok(header) => header,
        Err(error) => return Err(error),
    };
    let refresh_header = match set_cookie_header(
        cookie_config,
        REFRESH_SESSION_COOKIE_NAME,
        "",
        REFRESH_SESSION_COOKIE_PATH,
        0,
    ) {
        Ok(header) => header,
        Err(error) => return Err(error),
    };

    Ok(vec![access_header, refresh_header])
}

pub fn append_set_cookie_headers(headers: &mut HeaderMap, values: Vec<HeaderValue>) {
    for value in values {
        headers.append(SET_COOKIE, value);
    }
}

fn set_cookie_header(
    cookie_config: &CookieConfig,
    name: &str,
    value: &str,
    path: &str,
    max_age_seconds: u64,
) -> Result<HeaderValue, CookieHeaderError> {
    let mut header = format!(
        "{name}={value}; Max-Age={max_age_seconds}; Path={path}; HttpOnly; SameSite={}",
        cookie_config.same_site.as_set_cookie_value()
    );
    if cookie_config.secure {
        header.push_str("; Secure");
    }

    match HeaderValue::from_str(&header) {
        Ok(value) => Ok(value),
        Err(error) => Err(CookieHeaderError {
            detail: format!("invalid Set-Cookie header: {error}"),
        }),
    }
}

fn cookie_value(headers: &HeaderMap, cookie_name: &str) -> Option<String> {
    for header_value in headers.get_all(COOKIE) {
        let parsed = match header_value.to_str() {
            Ok(parsed) => parsed,
            Err(_) => continue,
        };
        for cookie_part in parsed.split(';') {
            let trimmed = cookie_part.trim();
            let (name, value) = match trimmed.split_once('=') {
                Some(parts) => parts,
                None => continue,
            };
            if name == cookie_name {
                return Some(value.to_string());
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use axum::http::{HeaderMap, HeaderValue, header::COOKIE};

    use crate::init::server_config::http_security_config::{CookieConfig, CookieSameSitePolicy};

    use super::{
        ACCESS_TOKEN_COOKIE_NAME, access_token_from_headers, auth_cookie_headers,
        refresh_session_from_headers,
    };

    #[test]
    fn reads_named_cookie_values() {
        let mut headers = HeaderMap::new();
        headers.insert(
            COOKIE,
            HeaderValue::from_static("other=value; access_token=abc.123; refresh_session=def"),
        );

        assert_eq!(
            access_token_from_headers(&headers),
            Some("abc.123".to_string())
        );
        assert_eq!(
            refresh_session_from_headers(&headers),
            Some("def".to_string())
        );
    }

    #[test]
    fn auth_cookie_headers_are_httponly_and_secure_when_configured() {
        let config = CookieConfig {
            secure: true,
            same_site: CookieSameSitePolicy::Lax,
        };
        let headers = auth_cookie_headers(
            &config,
            "access",
            Duration::from_secs(60),
            "refresh",
            Duration::from_secs(120),
        );

        assert!(headers.is_ok());
        let headers = match headers {
            Ok(headers) => headers,
            Err(_) => return,
        };
        assert_eq!(headers.len(), 2);
        let first = match headers[0].to_str() {
            Ok(first) => first,
            Err(_) => return,
        };
        assert!(first.contains(ACCESS_TOKEN_COOKIE_NAME));
        assert!(first.contains("HttpOnly"));
        assert!(first.contains("SameSite=Lax"));
        assert!(first.contains("Secure"));
    }
}
