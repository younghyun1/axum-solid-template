use std::net::SocketAddr;

use axum::{
    Router,
    extract::State,
    http::{HeaderMap, StatusCode, Uri, header, uri::Authority},
    response::{IntoResponse, Redirect, Response},
    routing::get,
};

#[derive(Clone, Copy, Debug)]
pub struct RedirectPorts {
    pub http: u16,
    pub https: u16,
}

pub fn build_redirect_router(ports: RedirectPorts) -> Router {
    Router::new()
        .fallback(get(redirect_to_https))
        .with_state(ports)
}

pub fn redirect_socket_addr(port: u16) -> SocketAddr {
    SocketAddr::from(([0, 0, 0, 0], port))
}

async fn redirect_to_https(
    State(ports): State<RedirectPorts>,
    headers: HeaderMap,
    uri: Uri,
) -> Response {
    let host = match headers.get(header::HOST) {
        Some(value) => match value.to_str() {
            Ok(host) => host,
            Err(error) => {
                tracing::warn!(error = %error, "Invalid Host header for HTTP redirect");
                return StatusCode::BAD_REQUEST.into_response();
            }
        },
        None => {
            tracing::warn!("Missing Host header for HTTP redirect");
            return StatusCode::BAD_REQUEST.into_response();
        }
    };

    match make_https_uri(host, uri, ports.https) {
        Ok(https_uri) => Redirect::permanent(&https_uri.to_string()).into_response(),
        Err(error) => {
            tracing::warn!(error = %error, "Failed to build HTTPS redirect URI");
            StatusCode::BAD_REQUEST.into_response()
        }
    }
}

fn make_https_uri(host: &str, uri: Uri, https_port: u16) -> Result<Uri, RedirectError> {
    let authority = match host.parse::<Authority>() {
        Ok(authority) => authority,
        Err(error) => {
            return Err(RedirectError::Authority {
                error: error.to_string(),
            });
        }
    };

    let mut parts = uri.into_parts();
    parts.scheme = Some(axum::http::uri::Scheme::HTTPS);

    if parts.path_and_query.is_none() {
        parts.path_and_query = match "/".parse() {
            Ok(path_and_query) => Some(path_and_query),
            Err(error) => {
                return Err(RedirectError::PathAndQuery {
                    error: error.to_string(),
                });
            }
        };
    }

    let next_authority = format!("{}:{https_port}", authority.host());
    parts.authority = match next_authority.parse::<Authority>() {
        Ok(authority) => Some(authority),
        Err(error) => {
            return Err(RedirectError::Authority {
                error: error.to_string(),
            });
        }
    };

    match Uri::from_parts(parts) {
        Ok(uri) => Ok(uri),
        Err(error) => Err(RedirectError::Uri {
            error: error.to_string(),
        }),
    }
}

#[derive(Debug)]
enum RedirectError {
    Authority { error: String },
    PathAndQuery { error: String },
    Uri { error: String },
}

impl std::fmt::Display for RedirectError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RedirectError::Authority { error } => {
                write!(formatter, "invalid authority: {error}")
            }
            RedirectError::PathAndQuery { error } => {
                write!(formatter, "invalid path and query: {error}")
            }
            RedirectError::Uri { error } => write!(formatter, "invalid URI: {error}"),
        }
    }
}
