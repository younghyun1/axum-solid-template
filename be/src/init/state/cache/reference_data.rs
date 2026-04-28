use std::{fmt, time::Duration, time::Instant};

use diesel_async::RunQueryDsl;
use serde::Serialize;
use tracing::{error, info};
use utoipa::ToSchema;

use crate::{
    domain::iso::{IsoCountry, IsoCountrySubdivision, IsoCurrency, IsoLanguage},
    init::db_pool::{DbPool, DbPoolInitError, get_conn},
    schema::{iso_country, iso_country_subdivision, iso_currency, iso_language},
};

const ISO_COUNTRY_TABLE: &str = "iso_country";
const ISO_CURRENCY_TABLE: &str = "iso_currency";
const ISO_LANGUAGE_TABLE: &str = "iso_language";
const ISO_COUNTRY_SUBDIVISION_TABLE: &str = "iso_country_subdivision";

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ReferenceDataCache {
    pub countries: Vec<IsoCountry>,
    pub currencies: Vec<IsoCurrency>,
    pub languages: Vec<IsoLanguage>,
    pub country_subdivisions: Vec<IsoCountrySubdivision>,
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
        })
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
        }
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
