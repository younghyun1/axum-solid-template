use diesel::{Queryable, Selectable};
use serde::Serialize;
use utoipa::ToSchema;

use crate::schema::{iso_country, iso_country_subdivision, iso_currency, iso_language};

#[derive(Debug, Clone, Queryable, Selectable, Serialize, ToSchema)]
#[diesel(table_name = iso_country)]
pub struct IsoCountry {
    pub country_code: i32,
    pub country_alpha2: String,
    pub country_alpha3: String,
    pub country_eng_name: String,
    pub country_primary_language: i32,
    pub country_currency: i32,
    pub phone_prefix: String,
    pub country_flag: String,
    pub is_country: bool,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, ToSchema)]
#[diesel(table_name = iso_currency)]
pub struct IsoCurrency {
    pub currency_code: i32,
    pub currency_alpha3: String,
    pub currency_name: String,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, ToSchema)]
#[diesel(table_name = iso_language)]
pub struct IsoLanguage {
    pub language_code: i32,
    pub language_alpha2: String,
    pub language_alpha3: String,
    pub language_eng_name: String,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, ToSchema)]
#[diesel(table_name = iso_country_subdivision)]
pub struct IsoCountrySubdivision {
    pub subdivision_id: i32,
    pub country_code: i32,
    pub subdivision_code: String,
    pub subdivision_name: String,
    pub subdivision_type: Option<String>,
}
