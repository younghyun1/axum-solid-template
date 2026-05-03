use crate::domain::iso::{IsoCountry, IsoCountrySubdivision, IsoCurrency, IsoLanguage};

use super::types::ReferenceDataCache;

impl ReferenceDataCache {
    /// Perform the `countries` operation as implemented by this function.
    ///
    /// # Arguments
    /// * `self` -
    /// # Returns
    /// Returns the value produced by this function.
    pub fn countries(&self) -> Vec<IsoCountry> {
        let mut countries = Vec::new();
        self.countries.iter_sync(|_, country| {
            countries.push(country.clone());
            true
        });
        countries
    }

    /// Perform the `currencies` operation as implemented by this function.
    ///
    /// # Arguments
    /// * `self` -
    /// # Returns
    /// Returns the value produced by this function.
    pub fn currencies(&self) -> Vec<IsoCurrency> {
        let mut currencies = Vec::new();
        self.currencies.iter_sync(|_, currency| {
            currencies.push(currency.clone());
            true
        });
        currencies
    }

    /// Perform the `languages` operation as implemented by this function.
    ///
    /// # Arguments
    /// * `self` -
    /// # Returns
    /// Returns the value produced by this function.
    pub fn languages(&self) -> Vec<IsoLanguage> {
        let mut languages = Vec::new();
        self.languages.iter_sync(|_, language| {
            languages.push(language.clone());
            true
        });
        languages
    }

    /// Perform the `all_country_subdivisions` operation as implemented by this function.
    ///
    /// # Arguments
    /// * `self` -
    /// # Returns
    /// Returns the value produced by this function.
    pub fn all_country_subdivisions(&self) -> Vec<IsoCountrySubdivision> {
        let mut country_subdivisions = Vec::new();
        self.country_subdivisions.iter_sync(|_, subdivision| {
            country_subdivisions.push(subdivision.clone());
            true
        });
        country_subdivisions
    }
}
