use serde::Serialize;
use utoipa::ToSchema;

use crate::domain::iso::{IsoCountry, IsoCountrySubdivision, IsoLanguage};

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ReferenceCountryResponse {
    pub country_code: i32,
    pub country_alpha2: String,
    pub country_alpha3: String,
    pub country_name: String,
    pub country_primary_language: i32,
    pub country_currency: i32,
    pub phone_prefix: String,
    pub country_flag: String,
    pub is_country: bool,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ReferenceLanguageResponse {
    pub language_code: i32,
    pub language_alpha2: String,
    pub language_alpha3: String,
    pub language_name: String,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ReferenceSubdivisionResponse {
    pub subdivision_id: i32,
    pub country_code: i32,
    pub subdivision_code: String,
    pub subdivision_name: String,
    pub subdivision_type: Option<String>,
    pub country_flag: String,
}

impl ReferenceCountryResponse {
    /// Converts a domain-country model into the API country DTO shape.
    ///
    /// # Arguments
    /// * `country` -
    /// # Returns
    /// Returns the value produced by this function.
    pub fn from_domain(country: IsoCountry) -> Self {
        Self {
            country_code: country.country_code,
            country_alpha2: country.country_alpha2,
            country_alpha3: country.country_alpha3,
            country_name: country.country_eng_name,
            country_primary_language: country.country_primary_language,
            country_currency: country.country_currency,
            phone_prefix: country.phone_prefix,
            country_flag: country.country_flag,
            is_country: country.is_country,
        }
    }
}

impl ReferenceLanguageResponse {
    /// Converts a domain-language model into the API language DTO shape.
    ///
    /// # Arguments
    /// * `language` -
    /// # Returns
    /// Returns the value produced by this function.
    pub fn from_domain(language: IsoLanguage) -> Self {
        Self {
            language_code: language.language_code,
            language_alpha2: language.language_alpha2,
            language_alpha3: language.language_alpha3,
            language_name: language.language_eng_name,
        }
    }
}

impl ReferenceSubdivisionResponse {
    /// Converts a country subdivision domain model and country flag into an API DTO.
    ///
    /// # Arguments
    /// * `subdivision` -
    /// * `country_flag` -
    /// # Returns
    /// Returns the value produced by this function.
    pub fn from_domain(subdivision: IsoCountrySubdivision, country_flag: String) -> Self {
        Self {
            subdivision_id: subdivision.subdivision_id,
            country_code: subdivision.country_code,
            subdivision_code: subdivision.subdivision_code,
            subdivision_name: subdivision.subdivision_name,
            subdivision_type: subdivision.subdivision_type,
            country_flag,
        }
    }
}
