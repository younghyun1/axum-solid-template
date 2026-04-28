use std::time::Duration;

pub const JWT_AUTHORIZATION_HEADER_NAME: &str = "Authorization";
pub const JWT_BEARER_SCHEME: &str = "Bearer";
pub const JWT_MIN_SECRET_KEY_BYTES: usize = 64;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JwtConfig {
    pub issuer: JwtIssuer,
    pub access_token_duration: Duration,
    pub refresh_token_duration: Duration,
    pub secret_key: JwtSecretKey,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JwtIssuer(pub String);

#[derive(Clone, PartialEq, Eq)]
pub struct JwtSecretKey {
    pub(super) bytes: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JwtConfigError {
    EmptyIssuer,
    SecretKeyTooShort {
        minimum_bytes: usize,
        actual_bytes: usize,
    },
    AccessTokenDurationIsZero,
    RefreshTokenDurationIsZero,
    RefreshTokenDurationShorterThanAccessTokenDuration,
}
