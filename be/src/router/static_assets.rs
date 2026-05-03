use axum::{
    body::Body,
    http::{HeaderMap, HeaderValue, StatusCode, Uri, header},
    response::{IntoResponse, Response},
};
use bytes::Bytes;
use mime_guess::from_path;
use rust_embed::Embed;
use std::{collections::HashMap, sync::OnceLock};
use tracing::{info, warn};

use super::static_asset_encoding::{ContentCodingPreference, select_static_encoding};

#[derive(Embed)]
#[folder = "fe/"]
struct EmbeddedFrontend;

static FRONTEND_ASSETS: OnceLock<FrontendAssets> = OnceLock::new();

struct FrontendAssets {
    assets: HashMap<String, FrontendAsset>,
}

struct FrontendAsset {
    data: Bytes,
}

impl FrontendAssets {
    /// Loads embedded frontend files from `rust-embed` into memory for fast static serving.
    ///
    /// # Returns
    /// Returns the value produced by this function.
    fn load() -> Self {
        let mut assets = HashMap::new();

        for asset_path in EmbeddedFrontend::iter() {
            let asset_path_string = asset_path.to_string();
            match EmbeddedFrontend::get(asset_path_string.as_str()) {
                Some(content) => {
                    assets.insert(
                        asset_path_string,
                        FrontendAsset {
                            data: Bytes::copy_from_slice(content.data.as_ref()),
                        },
                    );
                }
                None => {
                    warn!(
                        asset_path = %asset_path_string,
                        "Embedded frontend asset listed by rust_embed was not readable"
                    );
                }
            }
        }

        info!(
            frontend_asset_count = assets.len(),
            "Loaded embedded frontend assets into memory"
        );

        Self { assets }
    }

    /// Looks up an asset by path in the in-memory embedded asset map.
    ///
    /// # Arguments
    /// * `self` -
    /// * `path` -
    /// # Returns
    /// Returns the value produced by this function.
    fn get(&self, path: &str) -> Option<&FrontendAsset> {
        self.assets.get(path)
    }
}

/// Lazily initializes and returns the singleton `FrontendAssets` cache.
///
/// # Returns
/// Returns the value produced by this function.
fn frontend_assets() -> &'static FrontendAssets {
    FRONTEND_ASSETS.get_or_init(FrontendAssets::load)
}

/// Resolves a compressed variant (`.zst`/`.gz`) of the asset when available.
///
/// # Arguments
/// * `path` -
/// * `coding` -
/// # Returns
/// Returns the value produced by this function.
fn serve_compressed_asset(path: &str, coding: ContentCodingPreference) -> Option<Response> {
    let (extension, encoding_name) = match coding {
        ContentCodingPreference::Zstd => (".zst", HeaderValue::from_static("zstd")),
        ContentCodingPreference::Gzip => (".gz", HeaderValue::from_static("gzip")),
        ContentCodingPreference::Identity => return None,
    };

    let compressed_path = format!("{path}{extension}");
    serve_asset(compressed_path.as_str(), path, Some(encoding_name))
}

/// Resolves the uncompressed asset and serves it when compression is unavailable.
///
/// # Arguments
/// * `path` -
/// # Returns
/// Returns the value produced by this function.
fn serve_uncompressed_asset(path: &str) -> Option<Response> {
    serve_asset(path, path, None)
}

fn serve_asset(
    asset_path: &str,
    source_path: &str,
    content_encoding: Option<HeaderValue>,
) -> Option<Response> {
    match frontend_assets().get(asset_path) {
        Some(asset) => {
            let mime = from_path(source_path).first_or_octet_stream();
            let mut response = Response::new(Body::from(asset.data.clone()));
            *response.status_mut() = StatusCode::OK;

            let headers = response.headers_mut();
            match HeaderValue::from_str(mime.as_ref()) {
                Ok(content_type) => {
                    headers.insert(header::CONTENT_TYPE, content_type);
                }
                Err(error) => {
                    warn!(
                        asset_path = %asset_path,
                        source_path = %source_path,
                        content_type = mime.as_ref(),
                        error = %error,
                        "Failed to build frontend asset Content-Type header"
                    );
                    headers.insert(
                        header::CONTENT_TYPE,
                        HeaderValue::from_static("application/octet-stream"),
                    );
                }
            }

            if let Some(encoding) = content_encoding {
                headers.insert(header::CONTENT_ENCODING, encoding);
            }
            headers.insert(header::CACHE_CONTROL, cache_control_value(source_path));
            headers.insert(header::VARY, HeaderValue::from_static("Accept-Encoding"));

            Some(response)
        }
        None => None,
    }
}

/// Returns cache-control headers based on asset location and mutability.
///
/// # Arguments
/// * `source_path` -
/// # Returns
/// Returns the value produced by this function.
fn cache_control_value(source_path: &str) -> HeaderValue {
    if source_path == "index.html" {
        return HeaderValue::from_static("no-cache");
    }

    if source_path.starts_with("assets/") {
        return HeaderValue::from_static("public, max-age=31536000, immutable");
    }

    HeaderValue::from_static("public, max-age=3600")
}

/// Handles frontend requests by selecting encoding, serving static bytes, or
/// falling back to `index.html` for SPA navigation.
///
/// # Arguments
/// * `uri` -
/// * `headers` -
/// # Returns
/// Returns the value produced by this function.
#[allow(clippy::single_match)]
pub async fn static_asset_handler(uri: Uri, headers: HeaderMap) -> Response {
    let requested_path = uri.path().trim_start_matches('/');
    let path = match requested_path.is_empty() {
        true => "index.html",
        false => requested_path,
    };

    let selected_encoding = select_static_encoding(&headers);

    match serve_compressed_asset(path, selected_encoding) {
        Some(response) => return response,
        None => {}
    }

    match serve_uncompressed_asset(path) {
        Some(response) => return response,
        None => {}
    }

    match serve_compressed_asset("index.html", selected_encoding) {
        Some(response) => return response,
        None => {}
    }

    match serve_uncompressed_asset("index.html") {
        Some(response) => return response,
        None => {}
    }

    warn!(
        requested_path = %path,
        "Frontend asset and SPA fallback were not found"
    );

    (
        StatusCode::NOT_FOUND,
        [(header::VARY, "Accept-Encoding")],
        "Not Found",
    )
        .into_response()
}
