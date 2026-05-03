use scc::HashMap;

use crate::domain::iso::{IsoCountry, IsoCountrySubdivision, IsoCurrency, IsoLanguage};

pub struct ReferenceDataCache {
    pub(super) countries: HashMap<i32, IsoCountry>,
    pub(super) currencies: HashMap<i32, IsoCurrency>,
    pub(super) languages: HashMap<i32, IsoLanguage>,
    pub(super) country_subdivisions: HashMap<i32, IsoCountrySubdivision>,
    pub(super) country_codes_by_alpha2: HashMap<String, i32>,
    pub(super) country_codes_by_alpha3: HashMap<String, i32>,
    pub(super) country_codes_by_english_name: HashMap<String, i32>,
    pub(super) currency_codes_by_alpha3: HashMap<String, i32>,
    pub(super) currency_codes_by_english_name: HashMap<String, Vec<i32>>,
    pub(super) language_codes_by_alpha2: HashMap<String, i32>,
    pub(super) language_codes_by_alpha3: HashMap<String, i32>,
    pub(super) language_codes_by_english_name: HashMap<String, i32>,
    pub(super) subdivision_ids_by_code: HashMap<(i32, String), i32>,
    pub(super) subdivision_ids_by_english_name: HashMap<(i32, String), Vec<i32>>,
    pub(super) subdivisions_by_country: HashMap<i32, Vec<i32>>,
}
