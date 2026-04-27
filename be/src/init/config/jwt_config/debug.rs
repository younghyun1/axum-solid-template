use std::fmt;

use super::jwt_config::{
    JwtSecretKey, JwtSigningKey, JwtVerificationKey, JwtVerificationKeyMaterial,
};

impl fmt::Debug for JwtSecretKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("JwtSecretKey")
            .field("bytes", &"[redacted]")
            .finish()
    }
}

impl fmt::Debug for JwtSigningKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JwtSigningKey::Hmac {
                key_id, algorithm, ..
            } => formatter
                .debug_struct("JwtSigningKey::Hmac")
                .field("key_id", key_id)
                .field("algorithm", algorithm)
                .field("secret_key", &"[redacted]")
                .finish(),
            JwtSigningKey::AsymmetricPem {
                key_id, algorithm, ..
            } => formatter
                .debug_struct("JwtSigningKey::AsymmetricPem")
                .field("key_id", key_id)
                .field("algorithm", algorithm)
                .field("private_key_pem", &"[redacted]")
                .field("public_key_pem", &"[redacted]")
                .finish(),
        }
    }
}

impl fmt::Debug for JwtVerificationKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("JwtVerificationKey")
            .field("key_id", &self.key_id)
            .field("algorithm", &self.algorithm)
            .field("key_material", &"[redacted]")
            .finish()
    }
}

impl fmt::Debug for JwtVerificationKeyMaterial {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JwtVerificationKeyMaterial::HmacSecret(_) => {
                formatter.write_str("JwtVerificationKeyMaterial::HmacSecret([redacted])")
            }
            JwtVerificationKeyMaterial::PublicKeyPem(_) => {
                formatter.write_str("JwtVerificationKeyMaterial::PublicKeyPem([redacted])")
            }
        }
    }
}
