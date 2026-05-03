use crate::domain::iso::{IsoCountry, IsoCountrySubdivision, IsoCurrency, IsoLanguage};

use super::types::ReferenceDataCache;

impl ReferenceDataCache {
    pub fn countries(&self) -> Vec<IsoCountry> {
        let mut countries = Vec::new();
        self.countries.iter_sync(|_, country| {
            countries.push(country.clone());
            true
        });
        countries
    }

    pub fn currencies(&self) -> Vec<IsoCurrency> {
        let mut currencies = Vec::new();
        self.currencies.iter_sync(|_, currency| {
            currencies.push(currency.clone());
            true
        });
        currencies
    }

    pub fn languages(&self) -> Vec<IsoLanguage> {
        let mut languages = Vec::new();
        self.languages.iter_sync(|_, language| {
            languages.push(language.clone());
            true
        });
        languages
    }

    pub fn all_country_subdivisions(&self) -> Vec<IsoCountrySubdivision> {
        let mut country_subdivisions = Vec::new();
        self.country_subdivisions.iter_sync(|_, subdivision| {
            country_subdivisions.push(subdivision.clone());
            true
        });
        country_subdivisions
    }
}
