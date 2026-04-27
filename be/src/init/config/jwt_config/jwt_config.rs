use std::time::Duration;

pub const JWT_AUTHORIZATION_HEADER_NAME: &str = "Authorization";
pub const JWT_BEARER_SCHEME: &str = "Bearer";
pub const JWT_MIN_HMAC_SECRET_BYTES: usize = 32;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JwtConfig {
    pub issuer: JwtIssuer,
    pub audiences: Vec<JwtAudience>,
    pub signing_key: JwtSigningKey,
    pub verification_keys: Vec<JwtVerificationKey>,
    pub access_token_ttl: Duration,
    pub refresh_token_ttl: Duration,
    pub clock_skew_leeway: Duration,
    pub max_token_age: Option<Duration>,
    pub reject_tokens_issued_before: Option<JwtUnixTimestamp>,
    pub require_expiration_time: bool,
    pub require_issued_at: bool,
    pub require_not_before: bool,
    pub require_jwt_id: bool,
    pub require_subject: bool,
    pub refresh_token_rotation_enabled: bool,
    pub authorization_header_name: String,
    pub bearer_scheme: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JwtIssuer(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JwtAudience(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JwtKeyId(pub String);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JwtUnixTimestamp(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JwtAlgorithm {
    Hs256,
    Hs384,
    Hs512,
    Rs256,
    Rs384,
    Rs512,
    Es256,
    Es384,
    EdDsa,
}

#[derive(Clone, PartialEq, Eq)]
pub struct JwtSecretKey {
    pub(super) bytes: Vec<u8>,
}

#[derive(Clone, PartialEq, Eq)]
pub enum JwtSigningKey {
    Hmac {
        key_id: JwtKeyId,
        algorithm: JwtAlgorithm,
        secret_key: JwtSecretKey,
    },
    AsymmetricPem {
        key_id: JwtKeyId,
        algorithm: JwtAlgorithm,
        private_key_pem: String,
        public_key_pem: String,
    },
}

#[derive(Clone, PartialEq, Eq)]
pub struct JwtVerificationKey {
    pub key_id: JwtKeyId,
    pub algorithm: JwtAlgorithm,
    pub key_material: JwtVerificationKeyMaterial,
}

#[derive(Clone, PartialEq, Eq)]
pub enum JwtVerificationKeyMaterial {
    HmacSecret(JwtSecretKey),
    PublicKeyPem(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JwtConfigError {
    EmptyIssuer,
    EmptyAudience,
    EmptyAudiences,
    EmptyKeyId,
    HmacSecretTooShort {
        minimum_bytes: usize,
        actual_bytes: usize,
    },
    HmacSecretUsedWithAsymmetricAlgorithm {
        algorithm: JwtAlgorithm,
    },
    AsymmetricKeyUsedWithHmacAlgorithm {
        algorithm: JwtAlgorithm,
    },
    EmptyPemKey,
    VerificationKeyAlgorithmMismatch {
        key_id: JwtKeyId,
        signing_algorithm: JwtAlgorithm,
        verification_algorithm: JwtAlgorithm,
    },
    AccessTokenTtlIsZero,
    RefreshTokenTtlIsZero,
    RefreshTokenTtlShorterThanAccessTokenTtl,
    AuthorizationHeaderNameIsEmpty,
    BearerSchemeIsEmpty,
}
