use super::jwt_config::{
    JWT_MIN_SECRET_KEY_BYTES, JwtConfig, JwtConfigError, JwtIssuer, JwtSecretKey,
};

impl JwtConfig {
    pub fn validate(&self) -> Result<(), JwtConfigError> {
        self.issuer.validate()?;
        self.secret_key.validate()?;

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
    pub fn validate(&self) -> Result<(), JwtConfigError> {
        if self.0.trim().is_empty() {
            return Err(JwtConfigError::EmptyIssuer);
        }

        Ok(())
    }
}

impl JwtSecretKey {
    pub fn new(bytes: Vec<u8>) -> Result<Self, JwtConfigError> {
        if bytes.len() < JWT_MIN_SECRET_KEY_BYTES {
            return Err(JwtConfigError::SecretKeyTooShort {
                minimum_bytes: JWT_MIN_SECRET_KEY_BYTES,
                actual_bytes: bytes.len(),
            });
        }

        Ok(Self { bytes })
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.bytes.as_slice()
    }

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
