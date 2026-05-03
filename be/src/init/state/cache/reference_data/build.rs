use std::hash::Hash;

use scc::HashMap;

use crate::domain::iso::{IsoCountry, IsoCountrySubdivision, IsoCurrency, IsoLanguage};

use super::{error::ReferenceDataCacheError, lookup::text_key, types::ReferenceDataCache};

impl ReferenceDataCache {
    pub(super) fn build(
        countries: &[IsoCountry],
        currencies: &[IsoCurrency],
        languages: &[IsoLanguage],
        country_subdivisions: &[IsoCountrySubdivision],
    ) -> Result<Self, ReferenceDataCacheError> {
        let cache = Self::empty();

        for country in countries {
            match insert_unique(
                &cache.countries,
                country.country_code,
                country.clone(),
                "countries",
            ) {
                Ok(()) => {}
                Err(error) => return Err(error),
            }
            match insert_unique(
                &cache.country_codes_by_alpha2,
                text_key(&country.country_alpha2),
                country.country_code,
                "country_codes_by_alpha2",
            ) {
                Ok(()) => {}
                Err(error) => return Err(error),
            }
            match insert_unique(
                &cache.country_codes_by_alpha3,
                text_key(&country.country_alpha3),
                country.country_code,
                "country_codes_by_alpha3",
            ) {
                Ok(()) => {}
                Err(error) => return Err(error),
            }
            match insert_unique(
                &cache.country_codes_by_english_name,
                text_key(&country.country_eng_name),
                country.country_code,
                "country_codes_by_english_name",
            ) {
                Ok(()) => {}
                Err(error) => return Err(error),
            }
        }

        for currency in currencies {
            match insert_unique(
                &cache.currencies,
                currency.currency_code,
                currency.clone(),
                "currencies",
            ) {
                Ok(()) => {}
                Err(error) => return Err(error),
            }
            match insert_unique(
                &cache.currency_codes_by_alpha3,
                text_key(&currency.currency_alpha3),
                currency.currency_code,
                "currency_codes_by_alpha3",
            ) {
                Ok(()) => {}
                Err(error) => return Err(error),
            }
            match insert_index_value(
                &cache.currency_codes_by_english_name,
                text_key(&currency.currency_name),
                currency.currency_code,
                "currency_codes_by_english_name",
            ) {
                Ok(()) => {}
                Err(error) => return Err(error),
            }
        }

        for language in languages {
            match insert_unique(
                &cache.languages,
                language.language_code,
                language.clone(),
                "languages",
            ) {
                Ok(()) => {}
                Err(error) => return Err(error),
            }
            match insert_unique(
                &cache.language_codes_by_alpha2,
                text_key(&language.language_alpha2),
                language.language_code,
                "language_codes_by_alpha2",
            ) {
                Ok(()) => {}
                Err(error) => return Err(error),
            }
            match insert_unique(
                &cache.language_codes_by_alpha3,
                text_key(&language.language_alpha3),
                language.language_code,
                "language_codes_by_alpha3",
            ) {
                Ok(()) => {}
                Err(error) => return Err(error),
            }
            match insert_unique(
                &cache.language_codes_by_english_name,
                text_key(&language.language_eng_name),
                language.language_code,
                "language_codes_by_english_name",
            ) {
                Ok(()) => {}
                Err(error) => return Err(error),
            }
        }

        for subdivision in country_subdivisions {
            match insert_unique(
                &cache.country_subdivisions,
                subdivision.subdivision_id,
                subdivision.clone(),
                "country_subdivisions",
            ) {
                Ok(()) => {}
                Err(error) => return Err(error),
            }
            match insert_unique(
                &cache.subdivision_ids_by_code,
                (
                    subdivision.country_code,
                    text_key(&subdivision.subdivision_code),
                ),
                subdivision.subdivision_id,
                "subdivision_ids_by_code",
            ) {
                Ok(()) => {}
                Err(error) => return Err(error),
            }
            match insert_index_value(
                &cache.subdivision_ids_by_english_name,
                (
                    subdivision.country_code,
                    text_key(&subdivision.subdivision_name),
                ),
                subdivision.subdivision_id,
                "subdivision_ids_by_english_name",
            ) {
                Ok(()) => {}
                Err(error) => return Err(error),
            }
            match cache.subdivisions_by_country.update_sync(
                &subdivision.country_code,
                |_, subdivision_ids| {
                    subdivision_ids.push(subdivision.subdivision_id);
                },
            ) {
                Some(()) => {}
                None => {
                    match cache
                        .subdivisions_by_country
                        .insert_sync(subdivision.country_code, vec![subdivision.subdivision_id])
                    {
                        Ok(()) => {}
                        Err(_) => {
                            return Err(ReferenceDataCacheError::DuplicateIndex {
                                index: "subdivisions_by_country",
                            });
                        }
                    }
                }
            }
        }

        Ok(cache)
    }

    /// Perform the `empty` operation as implemented by this function.
    ///
    /// # Returns
    /// Returns the value produced by this function.
    fn empty() -> Self {
        Self {
            countries: HashMap::default(),
            currencies: HashMap::default(),
            languages: HashMap::default(),
            country_subdivisions: HashMap::default(),
            country_codes_by_alpha2: HashMap::default(),
            country_codes_by_alpha3: HashMap::default(),
            country_codes_by_english_name: HashMap::default(),
            currency_codes_by_alpha3: HashMap::default(),
            currency_codes_by_english_name: HashMap::default(),
            language_codes_by_alpha2: HashMap::default(),
            language_codes_by_alpha3: HashMap::default(),
            language_codes_by_english_name: HashMap::default(),
            subdivision_ids_by_code: HashMap::default(),
            subdivision_ids_by_english_name: HashMap::default(),
            subdivisions_by_country: HashMap::default(),
        }
    }
}

fn insert_unique<K, V>(
    map: &HashMap<K, V>,
    key: K,
    value: V,
    index: &'static str,
) -> Result<(), ReferenceDataCacheError>
where
    K: Eq + Hash,
{
    match map.insert_sync(key, value) {
        Ok(()) => Ok(()),
        Err(_) => Err(ReferenceDataCacheError::DuplicateIndex { index }),
    }
}

fn insert_index_value<K>(
    map: &HashMap<K, Vec<i32>>,
    key: K,
    value: i32,
    index: &'static str,
) -> Result<(), ReferenceDataCacheError>
where
    K: Eq + Hash,
{
    match map.update_sync(&key, |_, values| {
        values.push(value);
    }) {
        Some(()) => Ok(()),
        None => match map.insert_sync(key, vec![value]) {
            Ok(()) => Ok(()),
            Err(_) => Err(ReferenceDataCacheError::DuplicateIndex { index }),
        },
    }
}
