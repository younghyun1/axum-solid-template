use std::time::Instant;

use diesel_async::RunQueryDsl;
use tracing::{error, info};

use crate::{
    domain::iso::{IsoCountry, IsoCountrySubdivision, IsoCurrency, IsoLanguage},
    init::db_pool::{DbPool, get_conn},
    schema::{iso_country, iso_country_subdivision, iso_currency, iso_language},
};

use super::{error::ReferenceDataCacheError, types::ReferenceDataCache};

const ISO_COUNTRY_TABLE: &str = "iso_country";
const ISO_CURRENCY_TABLE: &str = "iso_currency";
const ISO_LANGUAGE_TABLE: &str = "iso_language";
const ISO_COUNTRY_SUBDIVISION_TABLE: &str = "iso_country_subdivision";

impl ReferenceDataCache {
    /// Perform the `load` operation as implemented by this function.
    ///
    /// # Arguments
    /// * `db_pool` -
    /// # Returns
    /// A `Result`, either containing the function output or an error.
    pub async fn load(db_pool: &DbPool) -> Result<Self, ReferenceDataCacheError> {
        let started_at = Instant::now();
        let country_pool = db_pool.clone();
        let currency_pool = db_pool.clone();
        let language_pool = db_pool.clone();
        let subdivision_pool = db_pool.clone();

        // Reference tables are immutable at runtime, so load them in parallel once at startup.
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

/// Perform the `load_countries` operation as implemented by this function.
///
/// # Arguments
/// * `db_pool` -
/// # Returns
/// A `Result`, either containing the function output or an error.
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

/// Perform the `load_currencies` operation as implemented by this function.
///
/// # Arguments
/// * `db_pool` -
/// # Returns
/// A `Result`, either containing the function output or an error.
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

/// Perform the `load_languages` operation as implemented by this function.
///
/// # Arguments
/// * `db_pool` -
/// # Returns
/// A `Result`, either containing the function output or an error.
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
