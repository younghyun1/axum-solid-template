use std::{fmt, hash::Hash, time::Instant};

use diesel_async::RunQueryDsl;
use scc::HashMap;
use serde::{Serialize, Serializer, ser::SerializeStruct};
use tracing::{error, info};

use crate::{
    domain::iso::{IsoCountry, IsoCountrySubdivision, IsoCurrency, IsoLanguage},
    init::db_pool::{DbPool, DbPoolInitError, get_conn},
    schema::{iso_country, iso_country_subdivision, iso_currency, iso_language},
};

const ISO_COUNTRY_TABLE: &str = "iso_country";
const ISO_CURRENCY_TABLE: &str = "iso_currency";
const ISO_LANGUAGE_TABLE: &str = "iso_language";
const ISO_COUNTRY_SUBDIVISION_TABLE: &str = "iso_country_subdivision";

pub struct ReferenceDataCache {
    countries: HashMap<i32, IsoCountry>,
    currencies: HashMap<i32, IsoCurrency>,
    languages: HashMap<i32, IsoLanguage>,
    country_subdivisions: HashMap<i32, IsoCountrySubdivision>,
    country_codes_by_alpha2: HashMap<String, i32>,
    country_codes_by_alpha3: HashMap<String, i32>,
    country_codes_by_english_name: HashMap<String, i32>,
    currency_codes_by_alpha3: HashMap<String, i32>,
    currency_codes_by_english_name: HashMap<String, Vec<i32>>,
    language_codes_by_alpha2: HashMap<String, i32>,
    language_codes_by_alpha3: HashMap<String, i32>,
    language_codes_by_english_name: HashMap<String, i32>,
    subdivision_ids_by_code: HashMap<(i32, String), i32>,
    subdivision_ids_by_english_name: HashMap<(i32, String), Vec<i32>>,
    subdivisions_by_country: HashMap<i32, Vec<i32>>,
}

#[derive(Debug)]
pub enum ReferenceDataCacheError {
    DbPool {
        table: &'static str,
        error: DbPoolInitError,
    },
    Query {
        table: &'static str,
        error: String,
    },
    DuplicateIndex {
        index: &'static str,
    },
    TaskJoin {
        task: &'static str,
        error: String,
    },
}

impl ReferenceDataCache {
    pub async fn load(db_pool: &DbPool) -> Result<Self, ReferenceDataCacheError> {
        let started_at = Instant::now();
        let country_pool = db_pool.clone();
        let currency_pool = db_pool.clone();
        let language_pool = db_pool.clone();
        let subdivision_pool = db_pool.clone();

        let countries_task = tokio::spawn(async move { load_countries(&country_pool).await });
        let currencies_task = tokio::spawn(async move { load_currencies(&currency_pool).await });
        let languages_task = tokio::spawn(async move { load_languages(&language_pool).await });
        let country_subdivisions_task =
            tokio::spawn(async move { load_country_subdivisions(&subdivision_pool).await });

        let (countries, currencies, languages, country_subdivisions) = tokio::join!(
            countries_task,
            currencies_task,
            languages_task,
            country_subdivisions_task,
        );

        let countries = match resolve_reference_data_task(ISO_COUNTRY_TABLE, countries) {
            Ok(countries) => countries,
            Err(error) => return Err(error),
        };
        let currencies = match resolve_reference_data_task(ISO_CURRENCY_TABLE, currencies) {
            Ok(currencies) => currencies,
            Err(error) => return Err(error),
        };
        let languages = match resolve_reference_data_task(ISO_LANGUAGE_TABLE, languages) {
            Ok(languages) => languages,
            Err(error) => return Err(error),
        };
        let country_subdivisions = match resolve_reference_data_task(
            ISO_COUNTRY_SUBDIVISION_TABLE,
            country_subdivisions,
        ) {
            Ok(country_subdivisions) => country_subdivisions,
            Err(error) => return Err(error),
        };
        let table_load_elapsed = started_at.elapsed();
        let build_started_at = Instant::now();
        let cache = match Self::build(&countries, &currencies, &languages, &country_subdivisions) {
            Ok(cache) => cache,
            Err(error) => return Err(error),
        };
        let cache_build_elapsed = build_started_at.elapsed();

        info!(
            countries = countries.len(),
            currencies = currencies.len(),
            languages = languages.len(),
            country_subdivisions = country_subdivisions.len(),
            table_load_elapsed = ?table_load_elapsed,
            cache_build_elapsed = ?cache_build_elapsed,
            elapsed = ?started_at.elapsed(),
            "Loaded reference data cache"
        );

        Ok(cache)
    }

    fn build(
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

impl fmt::Debug for ReferenceDataCache {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ReferenceDataCache")
            .field("countries", &self.countries.len())
            .field("currencies", &self.currencies.len())
            .field("languages", &self.languages.len())
            .field("country_subdivisions", &self.country_subdivisions.len())
            .finish()
    }
}

impl Serialize for ReferenceDataCache {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let countries = self.countries();
        let currencies = self.currencies();
        let languages = self.languages();
        let country_subdivisions = self.all_country_subdivisions();

        let mut state = match serializer.serialize_struct("ReferenceDataCache", 4) {
            Ok(state) => state,
            Err(error) => return Err(error),
        };
        match state.serialize_field("countries", &countries) {
            Ok(()) => {}
            Err(error) => return Err(error),
        }
        match state.serialize_field("currencies", &currencies) {
            Ok(()) => {}
            Err(error) => return Err(error),
        }
        match state.serialize_field("languages", &languages) {
            Ok(()) => {}
            Err(error) => return Err(error),
        }
        match state.serialize_field("country_subdivisions", &country_subdivisions) {
            Ok(()) => {}
            Err(error) => return Err(error),
        }

        match state.end() {
            Ok(serialized) => Ok(serialized),
            Err(error) => Err(error),
        }
    }
}

impl fmt::Display for ReferenceDataCacheError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DbPool { table, error } => {
                write!(
                    formatter,
                    "failed to get database connection for `{table}`: {error}"
                )
            }
            Self::Query { table, error } => {
                write!(
                    formatter,
                    "failed to load `{table}` reference data: {error}"
                )
            }
            Self::DuplicateIndex { index } => {
                write!(formatter, "duplicate reference data cache index `{index}`")
            }
            Self::TaskJoin { task, error } => {
                write!(
                    formatter,
                    "reference data cache task `{task}` failed: {error}"
                )
            }
        }
    }
}

fn resolve_reference_data_task<T>(
    task: &'static str,
    result: Result<Result<T, ReferenceDataCacheError>, tokio::task::JoinError>,
) -> Result<T, ReferenceDataCacheError> {
    match result {
        Ok(task_result) => task_result,
        Err(error) => Err(ReferenceDataCacheError::TaskJoin {
            task,
            error: error.to_string(),
        }),
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

async fn load_countries(db_pool: &DbPool) -> Result<Vec<IsoCountry>, ReferenceDataCacheError> {
    let started_at = Instant::now();
    let mut connection = match get_conn(db_pool).await {
        Ok(connection) => connection,
        Err(error) => {
            error!(
                table = ISO_COUNTRY_TABLE,
                error = %error,
                "Failed to acquire reference data cache database connection"
            );
            return Err(ReferenceDataCacheError::DbPool {
                table: ISO_COUNTRY_TABLE,
                error,
            });
        }
    };

    let rows = match iso_country::table.load::<IsoCountry>(&mut connection).await {
        Ok(rows) => rows,
        Err(error) => {
            error!(
                table = ISO_COUNTRY_TABLE,
                error = %error,
                "Failed to load reference data cache table"
            );
            return Err(ReferenceDataCacheError::Query {
                table: ISO_COUNTRY_TABLE,
                error: error.to_string(),
            });
        }
    };

    info!(
        table = ISO_COUNTRY_TABLE,
        row_count = rows.len(),
        elapsed = ?started_at.elapsed(),
        "Loaded reference data cache table"
    );

    Ok(rows)
}

async fn load_currencies(db_pool: &DbPool) -> Result<Vec<IsoCurrency>, ReferenceDataCacheError> {
    let started_at = Instant::now();
    let mut connection = match get_conn(db_pool).await {
        Ok(connection) => connection,
        Err(error) => {
            error!(
                table = ISO_CURRENCY_TABLE,
                error = %error,
                "Failed to acquire reference data cache database connection"
            );
            return Err(ReferenceDataCacheError::DbPool {
                table: ISO_CURRENCY_TABLE,
                error,
            });
        }
    };

    let rows = match iso_currency::table
        .load::<IsoCurrency>(&mut connection)
        .await
    {
        Ok(rows) => rows,
        Err(error) => {
            error!(
                table = ISO_CURRENCY_TABLE,
                error = %error,
                "Failed to load reference data cache table"
            );
            return Err(ReferenceDataCacheError::Query {
                table: ISO_CURRENCY_TABLE,
                error: error.to_string(),
            });
        }
    };

    info!(
        table = ISO_CURRENCY_TABLE,
        row_count = rows.len(),
        elapsed = ?started_at.elapsed(),
        "Loaded reference data cache table"
    );

    Ok(rows)
}

async fn load_languages(db_pool: &DbPool) -> Result<Vec<IsoLanguage>, ReferenceDataCacheError> {
    let started_at = Instant::now();
    let mut connection = match get_conn(db_pool).await {
        Ok(connection) => connection,
        Err(error) => {
            error!(
                table = ISO_LANGUAGE_TABLE,
                error = %error,
                "Failed to acquire reference data cache database connection"
            );
            return Err(ReferenceDataCacheError::DbPool {
                table: ISO_LANGUAGE_TABLE,
                error,
            });
        }
    };

    let rows = match iso_language::table
        .load::<IsoLanguage>(&mut connection)
        .await
    {
        Ok(rows) => rows,
        Err(error) => {
            error!(
                table = ISO_LANGUAGE_TABLE,
                error = %error,
                "Failed to load reference data cache table"
            );
            return Err(ReferenceDataCacheError::Query {
                table: ISO_LANGUAGE_TABLE,
                error: error.to_string(),
            });
        }
    };

    info!(
        table = ISO_LANGUAGE_TABLE,
        row_count = rows.len(),
        elapsed = ?started_at.elapsed(),
        "Loaded reference data cache table"
    );

    Ok(rows)
}

async fn load_country_subdivisions(
    db_pool: &DbPool,
) -> Result<Vec<IsoCountrySubdivision>, ReferenceDataCacheError> {
    let started_at = Instant::now();
    let mut connection = match get_conn(db_pool).await {
        Ok(connection) => connection,
        Err(error) => {
            error!(
                table = ISO_COUNTRY_SUBDIVISION_TABLE,
                error = %error,
                "Failed to acquire reference data cache database connection"
            );
            return Err(ReferenceDataCacheError::DbPool {
                table: ISO_COUNTRY_SUBDIVISION_TABLE,
                error,
            });
        }
    };

    let rows = match iso_country_subdivision::table
        .load::<IsoCountrySubdivision>(&mut connection)
        .await
    {
        Ok(rows) => rows,
        Err(error) => {
            error!(
                table = ISO_COUNTRY_SUBDIVISION_TABLE,
                error = %error,
                "Failed to load reference data cache table"
            );
            return Err(ReferenceDataCacheError::Query {
                table: ISO_COUNTRY_SUBDIVISION_TABLE,
                error: error.to_string(),
            });
        }
    };

    info!(
        table = ISO_COUNTRY_SUBDIVISION_TABLE,
        row_count = rows.len(),
        elapsed = ?started_at.elapsed(),
        "Loaded reference data cache table"
    );

    Ok(rows)
}

fn text_key(value: &str) -> String {
    value.trim().to_ascii_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn currency(currency_code: i32, currency_alpha3: &str, currency_name: &str) -> IsoCurrency {
        IsoCurrency {
            currency_code,
            currency_alpha3: String::from(currency_alpha3),
            currency_name: String::from(currency_name),
        }
    }

    fn subdivision(
        subdivision_id: i32,
        country_code: i32,
        subdivision_code: &str,
        subdivision_name: &str,
    ) -> IsoCountrySubdivision {
        IsoCountrySubdivision {
            subdivision_id,
            country_code,
            subdivision_code: String::from(subdivision_code),
            subdivision_name: String::from(subdivision_name),
            subdivision_type: None,
        }
    }

    #[test]
    fn currency_name_index_allows_ambiguous_iso_names() -> Result<(), String> {
        let currencies = vec![
            currency(112, "BYB", "Belarusian Ruble"),
            currency(933, "BYN", "Belarusian Ruble"),
            currency(840, "USD", "US Dollar"),
        ];
        let cache = match ReferenceDataCache::build(&[], &currencies, &[], &[]) {
            Ok(cache) => cache,
            Err(error) => return Err(error.to_string()),
        };

        assert_eq!(
            cache.currency_codes_by_english_name("belarusian ruble"),
            Some(vec![112, 933])
        );
        assert_eq!(
            cache.currency_code_by_english_name("Belarusian Ruble"),
            None
        );
        assert_eq!(cache.currency_code_by_english_name("US Dollar"), Some(840));
        Ok(())
    }

    #[test]
    fn subdivision_name_index_allows_ambiguous_iso_names() -> Result<(), String> {
        let subdivisions = vec![
            subdivision(12255, 4, "URU", "Uruzgān"),
            subdivision(48898, 4, "ORU", "Uruzgān"),
            subdivision(12267, 4, "BDS", "Badakhshān"),
        ];
        let cache = match ReferenceDataCache::build(&[], &[], &[], &subdivisions) {
            Ok(cache) => cache,
            Err(error) => return Err(error.to_string()),
        };

        assert_eq!(
            cache.subdivision_ids_by_english_name(4, "uruzgān"),
            Some(vec![12255, 48898])
        );
        assert_eq!(cache.subdivision_id_by_english_name(4, "Uruzgān"), None);
        assert_eq!(
            cache.subdivision_id_by_english_name(4, "Badakhshān"),
            Some(12267)
        );
        Ok(())
    }
}
