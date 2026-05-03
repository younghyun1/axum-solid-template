use crate::domain::iso::{IsoCountry, IsoCountrySubdivision, IsoCurrency, IsoLanguage};

use super::types::ReferenceDataCache;

impl ReferenceDataCache {
    pub fn country_by_code(&self, country_code: i32) -> Option<IsoCountry> {
        self.countries
            .read_sync(&country_code, |_, country| country.clone())
    }

    pub fn country_code_by_alpha2(&self, country_alpha2: &str) -> Option<i32> {
        self.country_codes_by_alpha2
            .read_sync(&text_key(country_alpha2), |_, country_code| *country_code)
    }

    pub fn country_code_by_alpha3(&self, country_alpha3: &str) -> Option<i32> {
        self.country_codes_by_alpha3
            .read_sync(&text_key(country_alpha3), |_, country_code| *country_code)
    }

    pub fn country_code_by_english_name(&self, country_eng_name: &str) -> Option<i32> {
        self.country_codes_by_english_name
            .read_sync(&text_key(country_eng_name), |_, country_code| *country_code)
    }

    pub fn country_by_alpha2(&self, country_alpha2: &str) -> Option<IsoCountry> {
        let country_code = match self.country_code_by_alpha2(country_alpha2) {
            Some(country_code) => country_code,
            None => return None,
        };

        self.country_by_code(country_code)
    }

    pub fn country_by_alpha3(&self, country_alpha3: &str) -> Option<IsoCountry> {
        let country_code = match self.country_code_by_alpha3(country_alpha3) {
            Some(country_code) => country_code,
            None => return None,
        };

        self.country_by_code(country_code)
    }

    pub fn country_by_english_name(&self, country_eng_name: &str) -> Option<IsoCountry> {
        let country_code = match self.country_code_by_english_name(country_eng_name) {
            Some(country_code) => country_code,
            None => return None,
        };

        self.country_by_code(country_code)
    }

    pub fn currency_by_code(&self, currency_code: i32) -> Option<IsoCurrency> {
        self.currencies
            .read_sync(&currency_code, |_, currency| currency.clone())
    }

    pub fn currency_code_by_alpha3(&self, currency_alpha3: &str) -> Option<i32> {
        self.currency_codes_by_alpha3
            .read_sync(&text_key(currency_alpha3), |_, currency_code| {
                *currency_code
            })
    }

    pub fn currency_code_by_english_name(&self, currency_name: &str) -> Option<i32> {
        let currency_codes = match self.currency_codes_by_english_name(currency_name) {
            Some(currency_codes) => currency_codes,
            None => return None,
        };
        if currency_codes.len() != 1 {
            return None;
        }

        Some(currency_codes[0])
    }

    pub fn currency_codes_by_english_name(&self, currency_name: &str) -> Option<Vec<i32>> {
        self.currency_codes_by_english_name
            .read_sync(&text_key(currency_name), |_, currency_codes| {
                currency_codes.clone()
            })
    }

    pub fn currency_by_alpha3(&self, currency_alpha3: &str) -> Option<IsoCurrency> {
        let currency_code = match self.currency_code_by_alpha3(currency_alpha3) {
            Some(currency_code) => currency_code,
            None => return None,
        };

        self.currency_by_code(currency_code)
    }

    pub fn currency_by_english_name(&self, currency_name: &str) -> Option<IsoCurrency> {
        let currency_code = match self.currency_code_by_english_name(currency_name) {
            Some(currency_code) => currency_code,
            None => return None,
        };

        self.currency_by_code(currency_code)
    }

    pub fn language_by_code(&self, language_code: i32) -> Option<IsoLanguage> {
        self.languages
            .read_sync(&language_code, |_, language| language.clone())
    }

    pub fn language_code_by_alpha2(&self, language_alpha2: &str) -> Option<i32> {
        self.language_codes_by_alpha2
            .read_sync(&text_key(language_alpha2), |_, language_code| {
                *language_code
            })
    }

    pub fn language_code_by_alpha3(&self, language_alpha3: &str) -> Option<i32> {
        self.language_codes_by_alpha3
            .read_sync(&text_key(language_alpha3), |_, language_code| {
                *language_code
            })
    }

    pub fn language_code_by_english_name(&self, language_eng_name: &str) -> Option<i32> {
        self.language_codes_by_english_name
            .read_sync(&text_key(language_eng_name), |_, language_code| {
                *language_code
            })
    }

    pub fn language_by_alpha2(&self, language_alpha2: &str) -> Option<IsoLanguage> {
        let language_code = match self.language_code_by_alpha2(language_alpha2) {
            Some(language_code) => language_code,
            None => return None,
        };

        self.language_by_code(language_code)
    }

    pub fn language_by_alpha3(&self, language_alpha3: &str) -> Option<IsoLanguage> {
        let language_code = match self.language_code_by_alpha3(language_alpha3) {
            Some(language_code) => language_code,
            None => return None,
        };

        self.language_by_code(language_code)
    }

    pub fn language_by_english_name(&self, language_eng_name: &str) -> Option<IsoLanguage> {
        let language_code = match self.language_code_by_english_name(language_eng_name) {
            Some(language_code) => language_code,
            None => return None,
        };

        self.language_by_code(language_code)
    }

    pub fn subdivision_by_id(&self, subdivision_id: i32) -> Option<IsoCountrySubdivision> {
        self.country_subdivisions
            .read_sync(&subdivision_id, |_, subdivision| subdivision.clone())
    }

    pub fn subdivision_id_by_code(&self, country_code: i32, subdivision_code: &str) -> Option<i32> {
        self.subdivision_ids_by_code.read_sync(
            &(country_code, text_key(subdivision_code)),
            |_, subdivision_id| *subdivision_id,
        )
    }

    pub fn subdivision_id_by_english_name(
        &self,
        country_code: i32,
        subdivision_name: &str,
    ) -> Option<i32> {
        let subdivision_ids =
            match self.subdivision_ids_by_english_name(country_code, subdivision_name) {
                Some(subdivision_ids) => subdivision_ids,
                None => return None,
            };
        if subdivision_ids.len() != 1 {
            return None;
        }

        Some(subdivision_ids[0])
    }

    pub fn subdivision_ids_by_english_name(
        &self,
        country_code: i32,
        subdivision_name: &str,
    ) -> Option<Vec<i32>> {
        self.subdivision_ids_by_english_name.read_sync(
            &(country_code, text_key(subdivision_name)),
            |_, subdivision_ids| subdivision_ids.clone(),
        )
    }

    pub fn subdivision_by_code(
        &self,
        country_code: i32,
        subdivision_code: &str,
    ) -> Option<IsoCountrySubdivision> {
        let subdivision_id = match self.subdivision_id_by_code(country_code, subdivision_code) {
            Some(subdivision_id) => subdivision_id,
            None => return None,
        };

        self.subdivision_by_id(subdivision_id)
    }

    pub fn subdivision_by_english_name(
        &self,
        country_code: i32,
        subdivision_name: &str,
    ) -> Option<IsoCountrySubdivision> {
        let subdivision_id =
            match self.subdivision_id_by_english_name(country_code, subdivision_name) {
                Some(subdivision_id) => subdivision_id,
                None => return None,
            };

        self.subdivision_by_id(subdivision_id)
    }

    pub fn country_currency(&self, country_code: i32) -> Option<IsoCurrency> {
        let country = match self.country_by_code(country_code) {
            Some(country) => country,
            None => return None,
        };

        self.currency_by_code(country.country_currency)
    }

    pub fn country_primary_language(&self, country_code: i32) -> Option<IsoLanguage> {
        let country = match self.country_by_code(country_code) {
            Some(country) => country,
            None => return None,
        };

        self.language_by_code(country.country_primary_language)
    }

    pub fn subdivision_country(&self, subdivision_id: i32) -> Option<IsoCountry> {
        let subdivision = match self.subdivision_by_id(subdivision_id) {
            Some(subdivision) => subdivision,
            None => return None,
        };

        self.country_by_code(subdivision.country_code)
    }

    pub fn country_subdivisions(&self, country_code: i32) -> Vec<IsoCountrySubdivision> {
        let subdivision_ids = match self
            .subdivisions_by_country
            .read_sync(&country_code, |_, subdivision_ids| subdivision_ids.clone())
        {
            Some(subdivision_ids) => subdivision_ids,
            None => return Vec::new(),
        };
        let mut subdivisions = Vec::with_capacity(subdivision_ids.len());

        for subdivision_id in subdivision_ids {
            let subdivision = match self.subdivision_by_id(subdivision_id) {
                Some(subdivision) => subdivision,
                None => continue,
            };
            subdivisions.push(subdivision);
        }

        subdivisions
    }
}

pub(super) fn text_key(value: &str) -> String {
    value.trim().to_ascii_lowercase()
}
