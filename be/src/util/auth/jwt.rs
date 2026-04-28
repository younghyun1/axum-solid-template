use std::fmt;

use chrono::{DateTime, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use uuid::Uuid;

use crate::{
    domain::auth::{
        jwt::{AccessTokenClaims, JwtTokenType},
        role::RoleType,
        user::User,
    },
    init::server_config::jwt_config::jwt_config::JwtConfig,
};

pub const JWT_ALGORITHM: Algorithm = Algorithm::HS512;
pub const JWT_BEARER_TOKEN_TYPE: &str = "Bearer";

#[derive(Debug, Clone)]
pub struct JwtUserContext {
    pub user: User,
    pub role_type: RoleType,
    pub role_name: String,
}

#[derive(Debug, Clone)]
pub struct IssuedAccessToken {
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub claims: AccessTokenClaims,
}

#[derive(Debug)]
pub enum JwtError {
    InvalidClock,
    Encode { error: String },
    Decode { error: String },
}

impl fmt::Display for JwtError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidClock => formatter.write_str("system clock produced invalid JWT time"),
            Self::Encode { error } => write!(formatter, "failed to encode JWT: {error}"),
            Self::Decode { error } => write!(formatter, "failed to decode JWT: {error}"),
        }
    }
}

pub fn issue_access_token(
    jwt_config: &JwtConfig,
    user_context: JwtUserContext,
) -> Result<IssuedAccessToken, JwtError> {
    let issued_at = Utc::now();
    let expires_at = match chrono::Duration::from_std(jwt_config.access_token_duration) {
        Ok(duration) => issued_at + duration,
        Err(_) => return Err(JwtError::InvalidClock),
    };

    let issued_at_timestamp = issued_at.timestamp();
    let expires_at_timestamp = expires_at.timestamp();
    if issued_at_timestamp < 0 || expires_at_timestamp < 0 {
        return Err(JwtError::InvalidClock);
    }

    let role_type = user_context.role_type;
    let user = user_context.user;
    let claims = AccessTokenClaims {
        iss: jwt_config.issuer.0.clone(),
        sub: user.user_id,
        aud: vec![jwt_audience(jwt_config)],
        exp: expires_at_timestamp,
        nbf: issued_at_timestamp,
        iat: issued_at_timestamp,
        jti: Uuid::now_v7(),
        token_type: JwtTokenType::Access,
        user_id: user.user_id,
        user_name: user.user_name,
        user_email: user.user_email,
        user_is_email_verified: user.user_is_email_verified,
        user_country: user.user_country,
        user_language: user.user_language,
        user_subdivision: user.user_subdivision,
        user_auth_token_version: user.user_auth_token_version,
        role_id: role_type.id(),
        role_name: user_context.role_name,
        role_type,
        role_access_level: role_type.access_level(),
        issued_at_iso: issued_at,
        expires_at_iso: expires_at,
    };

    let mut header = Header::new(JWT_ALGORITHM);
    header.typ = Some("JWT".to_string());

    let token = match encode(
        &header,
        &claims,
        &EncodingKey::from_secret(jwt_config.secret_key.as_bytes()),
    ) {
        Ok(token) => token,
        Err(error) => {
            return Err(JwtError::Encode {
                error: error.to_string(),
            });
        }
    };

    Ok(IssuedAccessToken {
        token,
        expires_at,
        claims,
    })
}

pub fn decode_access_token(
    jwt_config: &JwtConfig,
    token: &str,
) -> Result<AccessTokenClaims, JwtError> {
    let mut validation = Validation::new(JWT_ALGORITHM);
    validation.set_issuer(&[jwt_config.issuer.0.as_str()]);
    validation.set_audience(&[jwt_audience(jwt_config)]);
    validation.validate_nbf = true;
    validation.set_required_spec_claims(&["exp", "nbf", "iat", "iss", "sub", "aud"]);

    let token_data = match decode::<AccessTokenClaims>(
        token,
        &DecodingKey::from_secret(jwt_config.secret_key.as_bytes()),
        &validation,
    ) {
        Ok(token_data) => token_data,
        Err(error) => {
            return Err(JwtError::Decode {
                error: error.to_string(),
            });
        }
    };

    Ok(token_data.claims)
}

pub fn jwt_audience(jwt_config: &JwtConfig) -> String {
    format!("{}-api", jwt_config.issuer.0)
}
