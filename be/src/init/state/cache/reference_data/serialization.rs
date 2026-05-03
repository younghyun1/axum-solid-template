use std::fmt;

use serde::{Serialize, Serializer, ser::SerializeStruct};

use super::types::ReferenceDataCache;

impl fmt::Debug for ReferenceDataCache {
    /// Perform the `fmt` operation as implemented by this function.
    ///
    /// # Arguments
    /// * `self` -
    /// * `formatter` -
    /// # Returns
    /// Returns the value produced by this function.
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
