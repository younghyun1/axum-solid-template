use super::jwt_config::{
    JWT_MIN_SECRET_KEY_BYTES, JwtConfig, JwtConfigError, JwtIssuer, JwtSecretKey,
};

impl JwtConfig {
    /// Perform the `validate` operation as implemented by this function.
    ///
    /// # Arguments
    /// * `self) -> Result<(` -
    /// # Returns
    /// A `Result`, either containing the function output or an error.
    pub fn validate(&self) -> Result<(), JwtConfigError> {
        match self.issuer.validate() {
            Ok(()) => {}
            Err(error) => {
                return Err(error);
            }
        }

        match self.secret_key.validate() {
            Ok(()) => {}
            Err(error) => {
                return Err(error);
            }
        }

        if self.access_token_duration.is_zero() {
            return Err(JwtConfigError::AccessTokenDurationIsZero);
        }

        if self.refresh_token_duration.is_zero() {
            return Err(JwtConfigError::RefreshTokenDurationIsZero);
        }

        if self.refresh_token_duration < self.access_token_duration {
            return Err(JwtConfigError::RefreshTokenDurationShorterThanAccessTokenDuration);
        }

        Ok(())
    }
}

impl JwtIssuer {
    /// Perform the `validate` operation as implemented by this function.
    ///
    /// # Arguments
    /// * `self) -> Result<(` -
    /// # Returns
    /// A `Result`, either containing the function output or an error.
    pub fn validate(&self) -> Result<(), JwtConfigError> {
        if self.0.trim().is_empty() {
            return Err(JwtConfigError::EmptyIssuer);
        }

        Ok(())
    }
}

impl JwtSecretKey {
    /// Perform the `new` operation as implemented by this function.
    ///
    /// # Arguments
    /// * `bytes` -
    /// # Returns
    /// A `Result`, either containing the function output or an error.
    pub fn new(bytes: Vec<u8>) -> Result<Self, JwtConfigError> {
        if bytes.len() < JWT_MIN_SECRET_KEY_BYTES {
            return Err(JwtConfigError::SecretKeyTooShort {
                minimum_bytes: JWT_MIN_SECRET_KEY_BYTES,
                actual_bytes: bytes.len(),
            });
        }

        Ok(Self { bytes })
    }

    /// Perform the `as_bytes` operation as implemented by this function.
    ///
    /// # Arguments
    /// * `self` -
    /// # Returns
    /// Returns the value produced by this function.
    pub fn as_bytes(&self) -> &[u8] {
        self.bytes.as_slice()
    }

    /// Perform the `validate` operation as implemented by this function.
    ///
    /// # Arguments
    /// * `self) -> Result<(` -
    /// # Returns
    /// A `Result`, either containing the function output or an error.
    pub fn validate(&self) -> Result<(), JwtConfigError> {
        if self.bytes.len() < JWT_MIN_SECRET_KEY_BYTES {
            return Err(JwtConfigError::SecretKeyTooShort {
                minimum_bytes: JWT_MIN_SECRET_KEY_BYTES,
                actual_bytes: self.bytes.len(),
            });
        }

        Ok(())
    }
}
