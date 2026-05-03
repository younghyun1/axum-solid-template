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

impl AccessTokenClaims {
    /// Perform the `has_role` operation as implemented by this function.
    ///
    /// # Arguments
    /// * `self` -
    /// * `role_type` -
    /// # Returns
    /// Returns the value produced by this function.
    pub fn has_role(&self, role_type: RoleType) -> bool {
        self.role_type == role_type
    }

    /// Perform the `has_min_role` operation as implemented by this function.
    ///
    /// # Arguments
    /// * `self` -
    /// * `minimum_role` -
    /// # Returns
    /// Returns the value produced by this function.
    pub fn has_min_role(&self, minimum_role: RoleType) -> bool {
        self.role_type.has_min_access_level(minimum_role)
    }

    /// Perform the `is_admin` operation as implemented by this function.
    ///
    /// # Arguments
    /// * `self` -
    /// # Returns
    /// Returns the value produced by this function.
    pub fn is_admin(&self) -> bool {
        self.role_type.is_admin()
    }

    /// Perform the `is_moderator` operation as implemented by this function.
    ///
    /// # Arguments
    /// * `self` -
    /// # Returns
    /// Returns the value produced by this function.
    pub fn is_moderator(&self) -> bool {
        self.role_type.is_moderator()
    }

    /// Perform the `is_service_provider` operation as implemented by this function.
    ///
    /// # Arguments
    /// * `self` -
    /// # Returns
    /// Returns the value produced by this function.
    pub fn is_service_provider(&self) -> bool {
        self.role_type.is_service_provider()
    }

    /// Perform the `is_user_client` operation as implemented by this function.
    ///
    /// # Arguments
    /// * `self` -
    /// # Returns
    /// Returns the value produced by this function.
    pub fn is_user_client(&self) -> bool {
        self.role_type.is_user_client()
    }

    /// Perform the `is_guest` operation as implemented by this function.
    ///
    /// # Arguments
    /// * `self` -
    /// # Returns
    /// Returns the value produced by this function.
    pub fn is_guest(&self) -> bool {
        self.role_type.is_guest()
    }
}
