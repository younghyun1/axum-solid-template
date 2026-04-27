use super::jwt_config::{
    JWT_MIN_HMAC_SECRET_BYTES, JwtAlgorithm, JwtAudience, JwtConfig, JwtConfigError, JwtIssuer,
    JwtKeyId, JwtSecretKey, JwtSigningKey, JwtVerificationKey, JwtVerificationKeyMaterial,
};

impl JwtConfig {
    pub fn validate(&self) -> Result<(), JwtConfigError> {
        self.issuer.validate()?;

        if self.audiences.is_empty() {
            return Err(JwtConfigError::EmptyAudiences);
        }

        for audience in &self.audiences {
            audience.validate()?;
        }

        self.signing_key.validate()?;

        for verification_key in &self.verification_keys {
            verification_key.validate_for_signing_key(&self.signing_key)?;
        }

        if self.access_token_ttl.is_zero() {
            return Err(JwtConfigError::AccessTokenTtlIsZero);
        }

        if self.refresh_token_ttl.is_zero() {
            return Err(JwtConfigError::RefreshTokenTtlIsZero);
        }

        if self.refresh_token_ttl < self.access_token_ttl {
            return Err(JwtConfigError::RefreshTokenTtlShorterThanAccessTokenTtl);
        }

        if self.authorization_header_name.trim().is_empty() {
            return Err(JwtConfigError::AuthorizationHeaderNameIsEmpty);
        }

        if self.bearer_scheme.trim().is_empty() {
            return Err(JwtConfigError::BearerSchemeIsEmpty);
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

impl JwtAudience {
    pub fn validate(&self) -> Result<(), JwtConfigError> {
        if self.0.trim().is_empty() {
            return Err(JwtConfigError::EmptyAudience);
        }

        Ok(())
    }
}

impl JwtKeyId {
    pub fn validate(&self) -> Result<(), JwtConfigError> {
        if self.0.trim().is_empty() {
            return Err(JwtConfigError::EmptyKeyId);
        }

        Ok(())
    }
}

impl JwtSecretKey {
    pub fn new(bytes: Vec<u8>) -> Result<Self, JwtConfigError> {
        if bytes.len() < JWT_MIN_HMAC_SECRET_BYTES {
            return Err(JwtConfigError::HmacSecretTooShort {
                minimum_bytes: JWT_MIN_HMAC_SECRET_BYTES,
                actual_bytes: bytes.len(),
            });
        }

        Ok(Self { bytes })
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.bytes.as_slice()
    }
}

impl JwtSigningKey {
    pub fn key_id(&self) -> &JwtKeyId {
        match self {
            JwtSigningKey::Hmac { key_id, .. } => key_id,
            JwtSigningKey::AsymmetricPem { key_id, .. } => key_id,
        }
    }

    pub fn algorithm(&self) -> JwtAlgorithm {
        match self {
            JwtSigningKey::Hmac { algorithm, .. } => *algorithm,
            JwtSigningKey::AsymmetricPem { algorithm, .. } => *algorithm,
        }
    }

    pub fn validate(&self) -> Result<(), JwtConfigError> {
        match self {
            JwtSigningKey::Hmac {
                key_id,
                algorithm,
                secret_key,
            } => {
                key_id.validate()?;

                if !algorithm.is_hmac() {
                    return Err(JwtConfigError::HmacSecretUsedWithAsymmetricAlgorithm {
                        algorithm: *algorithm,
                    });
                }

                if secret_key.as_bytes().len() < JWT_MIN_HMAC_SECRET_BYTES {
                    return Err(JwtConfigError::HmacSecretTooShort {
                        minimum_bytes: JWT_MIN_HMAC_SECRET_BYTES,
                        actual_bytes: secret_key.as_bytes().len(),
                    });
                }

                Ok(())
            }
            JwtSigningKey::AsymmetricPem {
                key_id,
                algorithm,
                private_key_pem,
                public_key_pem,
            } => {
                key_id.validate()?;

                if algorithm.is_hmac() {
                    return Err(JwtConfigError::AsymmetricKeyUsedWithHmacAlgorithm {
                        algorithm: *algorithm,
                    });
                }

                if private_key_pem.trim().is_empty() || public_key_pem.trim().is_empty() {
                    return Err(JwtConfigError::EmptyPemKey);
                }

                Ok(())
            }
        }
    }
}

impl JwtVerificationKey {
    pub fn validate_for_signing_key(
        &self,
        signing_key: &JwtSigningKey,
    ) -> Result<(), JwtConfigError> {
        self.key_id.validate()?;

        if self.algorithm != signing_key.algorithm() {
            return Err(JwtConfigError::VerificationKeyAlgorithmMismatch {
                key_id: self.key_id.clone(),
                signing_algorithm: signing_key.algorithm(),
                verification_algorithm: self.algorithm,
            });
        }

        match &self.key_material {
            JwtVerificationKeyMaterial::HmacSecret(secret_key) => {
                if !self.algorithm.is_hmac() {
                    return Err(JwtConfigError::HmacSecretUsedWithAsymmetricAlgorithm {
                        algorithm: self.algorithm,
                    });
                }

                if secret_key.as_bytes().len() < JWT_MIN_HMAC_SECRET_BYTES {
                    return Err(JwtConfigError::HmacSecretTooShort {
                        minimum_bytes: JWT_MIN_HMAC_SECRET_BYTES,
                        actual_bytes: secret_key.as_bytes().len(),
                    });
                }

                Ok(())
            }
            JwtVerificationKeyMaterial::PublicKeyPem(public_key_pem) => {
                if self.algorithm.is_hmac() {
                    return Err(JwtConfigError::AsymmetricKeyUsedWithHmacAlgorithm {
                        algorithm: self.algorithm,
                    });
                }

                if public_key_pem.trim().is_empty() {
                    return Err(JwtConfigError::EmptyPemKey);
                }

                Ok(())
            }
        }
    }
}

impl JwtAlgorithm {
    pub fn is_hmac(self) -> bool {
        match self {
            JwtAlgorithm::Hs256 | JwtAlgorithm::Hs384 | JwtAlgorithm::Hs512 => true,
            JwtAlgorithm::Rs256
            | JwtAlgorithm::Rs384
            | JwtAlgorithm::Rs512
            | JwtAlgorithm::Es256
            | JwtAlgorithm::Es384
            | JwtAlgorithm::EdDsa => false,
        }
    }
}
