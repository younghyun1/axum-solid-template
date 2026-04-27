use std::fmt;

use super::jwt_config::JwtSecretKey;

impl fmt::Debug for JwtSecretKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("JwtSecretKey")
            .field("bytes", &"[redacted]")
            .finish()
    }
}
