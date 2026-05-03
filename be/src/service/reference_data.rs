use std::sync::Arc;

use crate::{
    dto::{
        api_response::ApiResult,
        reference_data::{
            ReferenceCountryResponse, ReferenceLanguageResponse, ReferenceSubdivisionResponse,
        },
    },
    error::{api_error::ApiError, code_error::CodeError},
    init::state::server_state::ServerState,
};

/// Reads all country records from cache, filters to country entries, sorts by
/// display name, and maps them to API DTOs.
///
/// # Arguments
/// * `state` -
/// # Returns
/// A `Result`, either containing the function output or an error.
pub fn countries(state: Arc<ServerState>) -> ApiResult<Vec<ReferenceCountryResponse>> {
    let mut countries: Vec<ReferenceCountryResponse> = state
        .reference_data_cache
        .countries()
        .into_iter()
        .filter(|country| country.is_country)
        .map(ReferenceCountryResponse::from_domain)
        .collect();

    countries.sort_by(|left, right| left.country_name.cmp(&right.country_name));

    Ok(countries)
}

/// Reads all language records from cache, sorts them by language name, and maps
/// them to API DTOs.
///
/// # Arguments
/// * `state` -
/// # Returns
/// A `Result`, either containing the function output or an error.
pub fn languages(state: Arc<ServerState>) -> ApiResult<Vec<ReferenceLanguageResponse>> {
    let mut languages: Vec<ReferenceLanguageResponse> = state
        .reference_data_cache
        .languages()
        .into_iter()
        .map(ReferenceLanguageResponse::from_domain)
        .collect();

    languages.sort_by(|left, right| left.language_name.cmp(&right.language_name));

    Ok(languages)
}

/// Reads subdivisions for a country code, returning HTTP-friendly DTOs for callers.
///
/// # Arguments
/// * `state` - Shared server state containing reference data cache.
/// * `country_code` - ISO country code used to scope subdivision lookup.
/// # Returns
/// A sorted list of subdivision DTOs or `REFERENCE_DATA_NOT_FOUND` if the code is invalid.
pub fn country_subdivisions(
    state: Arc<ServerState>,
    country_code: i32,
) -> ApiResult<Vec<ReferenceSubdivisionResponse>> {
    let country = match state.reference_data_cache.country_by_code(country_code) {
        Some(country) => country,
        None => {
            return Err(ApiError::public(
                CodeError::REFERENCE_DATA_NOT_FOUND,
                "Country code was not found",
            ));
        }
    };
    let mut subdivisions: Vec<ReferenceSubdivisionResponse> = state
        .reference_data_cache
        .country_subdivisions(country_code)
        .into_iter()
        .map(|subdivision| {
            ReferenceSubdivisionResponse::from_domain(subdivision, country.country_flag.clone())
        })
        .collect();

    subdivisions.sort_by(|left, right| left.subdivision_name.cmp(&right.subdivision_name));

    Ok(subdivisions)
}
