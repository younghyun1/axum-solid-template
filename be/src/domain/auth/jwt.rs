use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::domain::auth::role::RoleType;

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct AccessTokenClaims {
    pub iss: String,
    pub sub: Uuid,
    pub aud: Vec<String>,
    pub exp: i64,
    pub nbf: i64,
    pub iat: i64,
    pub jti: Uuid,
    pub token_type: JwtTokenType,
    pub user_id: Uuid,
    pub user_name: String,
    pub user_email: String,
    pub user_is_email_verified: bool,
    pub user_country: i32,
    pub user_language: i32,
    pub user_subdivision: Option<i32>,
    pub user_auth_token_version: i32,
    pub role_id: Uuid,
    pub role_name: String,
    pub role_type: RoleType,
    pub role_access_level: u8,
    pub issued_at_iso: DateTime<Utc>,
    pub expires_at_iso: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum JwtTokenType {
    Access,
}
