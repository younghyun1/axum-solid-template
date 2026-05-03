use crate::domain::iso::{IsoCountrySubdivision, IsoCurrency};

use super::types::ReferenceDataCache;

/// Perform the `currency` operation as implemented by this function.
///
/// # Arguments
/// * `currency_code` -
/// * `currency_alpha3` -
/// * `currency_name` -
/// # Returns
/// Returns the value produced by this function.
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

/// Perform the `currency_name_index_allows_ambiguous_iso_names` operation as implemented by this function.
///
/// # Arguments
/// * `) -> Result<(` -
/// # Returns
/// A `Result`, either containing the function output or an error.
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

/// Perform the `subdivision_name_index_allows_ambiguous_iso_names` operation as implemented by this function.
///
/// # Arguments
/// * `) -> Result<(` -
/// # Returns
/// A `Result`, either containing the function output or an error.
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
