use std::fmt;

use super::jwt_config::JwtSecretKey;

impl fmt::Debug for JwtSecretKey {
    /// Perform the `fmt` operation as implemented by this function.
    ///
    /// # Arguments
    /// * `self` -
    /// * `formatter` -
    /// # Returns
    /// Returns the value produced by this function.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("JwtSecretKey")
            .field("bytes", &"[redacted]")
            .finish()
    }
}
