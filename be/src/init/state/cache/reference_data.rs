use std::{fmt, hash::Hash, time::Duration, time::Instant};

use diesel_async::RunQueryDsl;
scc::HashMap;
use serde::Serialize;
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

#[derive(Serialize)]
pub struct ReferenceDataCache {
    pub countries: Vec<IsoCountry>,
    pub currencies: Vec<IsoCurrency>,
    pub languages: Vec<IsoLanguage>,
    pub country_subdivisions: Vec<IsoCountrySubdivision>,
    #[serde(skip_serializing)]
    countries_by_code: HashMap<i32, IsoCountry>,
    #[serde(skip_serializing)]
    countries_by_alpha2: HashMap<String, IsoCountry>,
    #[serde(skip_serializing)]
    countries_by_alpha3: HashMap<String, IsoCountry>,
    #[serde(skip_serializing)]
    countries_by_english_name: HashMap<String, IsoCountry>,
    #[serde(skip_serializing)]
    currencies_by_code: HashMap<i32, IsoCurrency>,
    #[serde(skip_serializing)]
    currencies_by_alpha3: HashMap<String, IsoCurrency>,
    #[serde(skip_serializing)]
    currencies_by_english_name: HashMap<String, IsoCurrency>,
    #[serde(skip_serializing)]
    languages_by_code: HashMap<i32, IsoLanguage>,
    #[serde(skip_serializing)]
    languages_by_alpha2: HashMap<String, IsoLanguage>,
    #[serde(skip_serializing)]
    languages_by_alpha3: HashMap<String, IsoLanguage>,
    #[serde(skip_serializing)]
    languages_by_english_name: HashMap<String, IsoLanguage>,
    #[serde(skip_serializing)]
    subdivisions_by_id: HashMap<i32, IsoCountrySubdivision>,
    #[serde(skip_serializing)]
    subdivisions_by_code: HashMap<(i32, String), IsoCountrySubdivision>,
    #[serde(skip_serializing)]
    subdivisions_by_english_name: HashMap<(i32, String), IsoCountrySubdivision>,
    #[serde(skip_serializing)]
    subdivisions_by_country: HashMap<i32, Vec<IsoCountrySubdivision>>,
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
}

impl ReferenceDataCache {
    pub async fn load(db_pool: &DbPool) -> Result<Self, ReferenceDataCacheError> {
        let started_at = Instant::now();
        let (countries, currencies, languages, country_subdivisions) = tokio::join!(
            load_countries(db_pool),
            load_currencies(db_pool),
            load_languages(db_pool),
            load_country_subdivisions(db_pool),
        );

        let countries = match countries {
            Ok(countries) => countries,
            Err(error) => return Err(error),
        };
        let currencies = match currencies {
            Ok(currencies) => currencies,
            Err(error) => return Err(error),
        };
        let languages = match languages {
            Ok(languages) => languages,
            Err(error) => return Err(error),
        };
        let country_subdivisions = match country_subdivisions {
            Ok(country_subdivisions) => country_subdivisions,
            Err(error) => return Err(error),
        };
        let indexes = match ReferenceDataCacheIndexes::build(
            &countries,
            &currencies,
            &languages,
            &country_subdivisions,
        ) {
            Ok(indexes) => indexes,
            Err(error) => return Err(error),
        };

        info!(
            countries = countries.len(),
            currencies = currencies.len(),
            languages = languages.len(),
            country_subdivisions = country_subdivisions.len(),
            elapsed_ms = elapsed_ms(started_at.elapsed()),
            "Loaded reference data cache"
        );

        Ok(Self {
            countries,
            currencies,
            languages,
            country_subdivisions,
            countries_by_code: indexes.countries_by_code,
            countries_by_alpha2: indexes.countries_by_alpha2,
            countries_by_alpha3: indexes.countries_by_alpha3,
            countries_by_english_name: indexes.countries_by_english_name,
            currencies_by_code: indexes.currencies_by_code,
            currencies_by_alpha3: indexes.currencies_by_alpha3,
            currencies_by_english_name: indexes.currencies_by_english_name,
            languages_by_code: indexes.languages_by_code,
            languages_by_alpha2: indexes.languages_by_alpha2,
            languages_by_alpha3: indexes.languages_by_alpha3,
            languages_by_english_name: indexes.languages_by_english_name,
            subdivisions_by_id: indexes.subdivisions_by_id,
            subdivisions_by_code: indexes.subdivisions_by_code,
            subdivisions_by_english_name: indexes.subdivisions_by_english_name,
            subdivisions_by_country: indexes.subdivisions_by_country,
        })
    }

    pub fn country_by_code(&self, country_code: i32) -> Option<IsoCountry> {
        self.countries_by_code
            .read_sync(&country_code, |_, country| country.clone())
    }

    pub fn country_by_alpha2(&self, country_alpha2: &str) -> Option<IsoCountry> {
        self.countries_by_alpha2
            .read_sync(&text_key(country_alpha2), |_, country| country.clone())
    }

    pub fn country_by_alpha3(&self, country_alpha3: &str) -> Option<IsoCountry> {
        self.countries_by_alpha3
            .read_sync(&text_key(country_alpha3), |_, country| country.clone())
    }

    pub fn country_by_english_name(&self, country_eng_name: &str) -> Option<IsoCountry> {
        self.countries_by_english_name
            .read_sync(&text_key(country_eng_name), |_, country| country.clone())
    }

    pub fn currency_by_code(&self, currency_code: i32) -> Option<IsoCurrency> {
        self.currencies_by_code
            .read_sync(&currency_code, |_, currency| currency.clone())
    }

    pub fn currency_by_alpha3(&self, currency_alpha3: &str) -> Option<IsoCurrency> {
        self.currencies_by_alpha3
            .read_sync(&text_key(currency_alpha3), |_, currency| currency.clone())
    }

    pub fn currency_by_english_name(&self, currency_name: &str) -> Option<IsoCurrency> {
        self.currencies_by_english_name
            .read_sync(&text_key(currency_name), |_, currency| currency.clone())
    }

    pub fn language_by_code(&self, language_code: i32) -> Option<IsoLanguage> {
        self.languages_by_code
            .read_sync(&language_code, |_, language| language.clone())
    }

    pub fn language_by_alpha2(&self, language_alpha2: &str) -> Option<IsoLanguage> {
        self.languages_by_alpha2
            .read_sync(&text_key(language_alpha2), |_, language| language.clone())
    }

    pub fn language_by_alpha3(&self, language_alpha3: &str) -> Option<IsoLanguage> {
        self.languages_by_alpha3
            .read_sync(&text_key(language_alpha3), |_, language| language.clone())
    }

    pub fn language_by_english_name(&self, language_eng_name: &str) -> Option<IsoLanguage> {
        self.languages_by_english_name
            .read_sync(&text_key(language_eng_name), |_, language| language.clone())
    }

    pub fn subdivision_by_id(&self, subdivision_id: i32) -> Option<IsoCountrySubdivision> {
        self.subdivisions_by_id
            .read_sync(&subdivision_id, |_, subdivision| subdivision.clone())
    }

    pub fn subdivision_by_code(
        &self,
        country_code: i32,
        subdivision_code: &str,
    ) -> Option<IsoCountrySubdivision> {
        self.subdivisions_by_code.read_sync(
            &(country_code, text_key(subdivision_code)),
            |_, subdivision| subdivision.clone(),
        )
    }

    pub fn subdivision_by_english_name(
        &self,
        country_code: i32,
        subdivision_name: &str,
    ) -> Option<IsoCountrySubdivision> {
        self.subdivisions_by_english_name.read_sync(
            &(country_code, text_key(subdivision_name)),
            |_, subdivision| subdivision.clone(),
        )
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
        match self
            .subdivisions_by_country
            .read_sync(&country_code, |_, subdivisions| subdivisions.clone())
        {
            Some(subdivisions) => subdivisions,
            None => Vec::new(),
        }
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

impl fmt::Display for ReferenceDataCacheError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DbPool { table, error } => {
                write!(formatter, "failed to get database connection for `{table}`: {error}")
            }
            Self::Query { table, error } => {
                write!(formatter, "failed to load `{table}` reference data: {error}")
            }
            Self::DuplicateIndex { index } => {
                write!(formatter, "duplicate reference data cache index `{index}`")
            }
        }
    }
}

struct ReferenceDataCacheIndexes {
    countries_by_code: HashMap<i32, IsoCountry>,
    countries_by_alpha2: HashMap<String, IsoCountry>,
    countries_by_alpha3: HashMap<String, IsoCountry>,
    countries_by_english_name: HashMap<String, IsoCountry>,
    currencies_by_code: HashMap<i32, IsoCurrency>,
    currencies_by_alpha3: HashMap<String, IsoCurrency>,
    currencies_by_english_name: HashMap<String, IsoCurrency>,
    languages_by_code: HashMap<i32, IsoLanguage>,
    languages_by_alpha2: HashMap<String, IsoLanguage>,
    languages_by_alpha3: HashMap<String, IsoLanguage>,
    languages_by_english_name: HashMap<String, IsoLanguage>,
    subdivisions_by_id: HashMap<i32, IsoCountrySubdivision>,
    subdivisions_by_code: HashMap<(i32, String), IsoCountrySubdivision>,
    subdivisions_by_english_name: HashMap<(i32, String), IsoCountrySubdivision>,
    subdivisions_by_country: HashMap<i32, Vec<IsoCountrySubdivision>>,
}

impl ReferenceDataCacheIndexes {
    fn build(
        countries: &[IsoCountry],
        currencies: &[IsoCurrency],
        languages: &[IsoLanguage],
        country_subdivisions: &[IsoCountrySubdivision],
    ) -> Result<Self, ReferenceDataCacheError> {
        let indexes = Self::empty();

        for country in countries {
            match insert_unique(
                &indexes.countries_by_code,
                country.country_code,
                country.clone(),
                "countries_by_code",
            ) {
                Ok(()) => {}
                Err(error) => return Err(error),
            }
            match insert_unique(
                &indexes.countries_by_alpha2,
                text_key(&country.country_alpha2),
                country.clone(),
                "countries_by_alpha2",
            ) {
                Ok(()) => {}
                Err(error) => return Err(error),
            }
            match insert_unique(
                &indexes.countries_by_alpha3,
                text_key(&country.country_alpha3),
                country.clone(),
                "countries_by_alpha3",
            ) {
                Ok(()) => {}
                Err(error) => return Err(error),
            }
            match insert_unique(
                &indexes.countries_by_english_name,
                text_key(&country.country_eng_name),
                country.clone(),
                "countries_by_english_name",
            ) {
                Ok(()) => {}
                Err(error) => return Err(error),
            }
        }

        for currency in currencies {
            match insert_unique(
                &indexes.currencies_by_code,
                currency.currency_code,
                currency.clone(),
                "currencies_by_code",
            ) {
                Ok(()) => {}
                Err(error) => return Err(error),
            }
            match insert_unique(
                &indexes.currencies_by_alpha3,
                text_key(&currency.currency_alpha3),
                currency.clone(),
                "currencies_by_alpha3",
            ) {
                Ok(()) => {}
                Err(error) => return Err(error),
            }
            match insert_unique(
                &indexes.currencies_by_english_name,
                text_key(&currency.currency_name),
                currency.clone(),
                "currencies_by_english_name",
            ) {
                Ok(()) => {}
                Err(error) => return Err(error),
            }
        }

        for language in languages {
            match insert_unique(
                &indexes.languages_by_code,
                language.language_code,
                language.clone(),
                "languages_by_code",
            ) {
                Ok(()) => {}
                Err(error) => return Err(error),
            }
            match insert_unique(
                &indexes.languages_by_alpha2,
                text_key(&language.language_alpha2),
                language.clone(),
                "languages_by_alpha2",
            ) {
                Ok(()) => {}
                Err(error) => return Err(error),
            }
            match insert_unique(
                &indexes.languages_by_alpha3,
                text_key(&language.language_alpha3),
                language.clone(),
                "languages_by_alpha3",
            ) {
                Ok(()) => {}
                Err(error) => return Err(error),
            }
            match insert_unique(
                &indexes.languages_by_english_name,
                text_key(&language.language_eng_name),
                language.clone(),
                "languages_by_english_name",
            ) {
                Ok(()) => {}
                Err(error) => return Err(error),
            }
        }

        for subdivision in country_subdivisions {
            match insert_unique(
                &indexes.subdivisions_by_id,
                subdivision.subdivision_id,
                subdivision.clone(),
                "subdivisions_by_id",
            ) {
                Ok(()) => {}
                Err(error) => return Err(error),
            }
            match insert_unique(
                &indexes.subdivisions_by_code,
                (subdivision.country_code, text_key(&subdivision.subdivision_code)),
                subdivision.clone(),
                "subdivisions_by_code",
            ) {
                Ok(()) => {}
                Err(error) => return Err(error),
            }
            match insert_unique(
                &indexes.subdivisions_by_english_name,
                (subdivision.country_code, text_key(&subdivision.subdivision_name)),
                subdivision.clone(),
                "subdivisions_by_english_name",
            ) {
                Ok(()) => {}
                Err(error) => return Err(error),
            }
            match indexes
                .subdivisions_by_country
                .update_sync(&subdivision.country_code, |_, subdivisions| {
                    subdivisions.push(subdivision.clone());
                }) {
                Some(()) => {}
                None => {
                    match indexes.subdivisions_by_country.insert_sync(
                        subdivision.country_code,
                        vec![subdivision.clone()],
                    ) {
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

        Ok(indexes)
    }

    fn empty() -> Self {
        Self {
            countries_by_code: HashMap::default(),
            countries_by_alpha2: HashMap::default(),
            countries_by_alpha3: HashMap::default(),
            countries_by_english_name: HashMap::default(),
            currencies_by_code: HashMap::default(),
            currencies_by_alpha3: HashMap::default(),
            currencies_by_english_name: HashMap::default(),
            languages_by_code: HashMap::default(),
            languages_by_alpha2: HashMap::default(),
            languages_by_alpha3: HashMap::default(),
            languages_by_english_name: HashMap::default(),
            subdivisions_by_id: HashMap::default(),
            subdivisions_by_code: HashMap::default(),
            subdivisions_by_english_name: HashMap::default(),
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
        elapsed_ms = elapsed_ms(started_at.elapsed()),
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

    let rows = match iso_currency::table.load::<IsoCurrency>(&mut connection).await {
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
        elapsed_ms = elapsed_ms(started_at.elapsed()),
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

    let rows = match iso_language::table.load::<IsoLanguage>(&mut connection).await {
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
        elapsed_ms = elapsed_ms(started_at.elapsed()),
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
        elapsed_ms = elapsed_ms(started_at.elapsed()),
        "Loaded reference data cache table"
    );

    Ok(rows)
}

fn elapsed_ms(duration: Duration) -> u64 {
    match u64::try_from(duration.as_millis()) {
        Ok(elapsed) => elapsed,
        Err(_) => u64::MAX,
    }
}

fn text_key(value: &str) -> String {
    value.trim().to_ascii_lowercase()
}
