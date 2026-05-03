use std::sync::Arc;

use axum::extract::{Path, State};

use crate::{
    dto::{
        api_response::{ApiEnvelope, ApiResponseResult, ApiResult},
        reference_data::{
            ReferenceCountryResponse, ReferenceLanguageResponse, ReferenceSubdivisionResponse,
        },
    },
    error::prelude::*,
    init::state::server_state::ServerState,
    service::reference_data::{
        countries as countries_service, country_subdivisions as country_subdivisions_service,
        languages as languages_service,
    },
};

#[utoipa::path(
    get,
    path = "/api/v1/reference/countries",
    tag = "reference",
    responses((status = 200, description = "Cached country options", body = ApiEnvelope<Vec<ReferenceCountryResponse>>))
)]
/// Return the cached country reference dataset.
///
/// # Arguments
/// * `state` - Shared server state containing preloaded reference caches.
///
/// # Returns
/// `ApiResponseResult<Vec<ReferenceCountryResponse>>` with all configured countries.
pub async fn countries(
    State(state): State<Arc<ServerState>>,
) -> ApiResponseResult<Vec<ReferenceCountryResponse>> {
    response_from_result(countries_service(state))
}

#[utoipa::path(
    get,
    path = "/api/v1/reference/languages",
    tag = "reference",
    responses((status = 200, description = "Cached language options", body = ApiEnvelope<Vec<ReferenceLanguageResponse>>))
)]
/// Return the cached language reference dataset.
///
/// # Arguments
/// * `state` - Shared server state containing preloaded language caches.
///
/// # Returns
/// `ApiResponseResult<Vec<ReferenceLanguageResponse>>` with all configured languages.
pub async fn languages(
    State(state): State<Arc<ServerState>>,
) -> ApiResponseResult<Vec<ReferenceLanguageResponse>> {
    response_from_result(languages_service(state))
}

#[utoipa::path(
    get,
    path = "/api/v1/reference/countries/{country_code}/subdivisions",
    tag = "reference",
    params(("country_code" = i32, Path, description = "ISO numeric country code")),
    responses((status = 200, description = "Cached subdivisions for one country", body = ApiEnvelope<Vec<ReferenceSubdivisionResponse>>))
)]
/// Return subdivisions for a country from cached reference data.
///
/// # Arguments
/// * `state` - Shared server state with subdivision cache.
/// * `country_code` - ISO numeric code for the requested country.
///
/// # Returns
/// `ApiResponseResult<Vec<ReferenceSubdivisionResponse>>` for that country.
pub async fn country_subdivisions(
    State(state): State<Arc<ServerState>>,
    Path(country_code): Path<i32>,
) -> ApiResponseResult<Vec<ReferenceSubdivisionResponse>> {
    response_from_result(country_subdivisions_service(state, country_code))
}

/// Convert a repository/service result into an HTTP response shape used by controllers.
///
/// # Arguments
/// * `result` - Generic service result.
///
/// # Returns
/// `ApiResponseResult<T>` where successful values are wrapped as `api_ok`, errors are propagated.
fn response_from_result<T>(result: ApiResult<T>) -> ApiResponseResult<T> {
    match result {
        Ok(response) => Ok(api_ok(response)),
        Err(error) => Err(error),
    }
}
