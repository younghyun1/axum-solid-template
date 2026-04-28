use chrono::{DateTime, Utc};
use diesel::{Insertable, Queryable, Selectable};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::schema::{email_verification_tokens, password_reset_tokens, users};

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub user_id: Uuid,
    pub user_name: String,
    pub user_email: String,
    pub user_password_hash: String,
    pub user_created_at: DateTime<Utc>,
    pub user_updated_at: DateTime<Utc>,
    pub user_password_changed_at: DateTime<Utc>,
    pub user_last_login_at: Option<DateTime<Utc>>,
    pub user_is_email_verified: bool,
    pub user_auth_token_version: i32,
    pub user_country: i32,
    pub user_language: i32,
    pub user_subdivision: Option<i32>,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, ToSchema)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserInfo {
    pub user_id: Uuid,
    pub user_name: String,
    pub user_email: String,
    pub user_created_at: DateTime<Utc>,
    pub user_updated_at: DateTime<Utc>,
    pub user_last_login_at: Option<DateTime<Utc>>,
    pub user_is_email_verified: bool,
    pub user_country: i32,
    pub user_language: i32,
    pub user_subdivision: Option<i32>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub user_name: String,
    pub user_email: String,
    pub user_password_hash: String,
    pub user_country: i32,
    pub user_language: i32,
    pub user_subdivision: Option<i32>,
}

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = email_verification_tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct EmailVerificationToken {
    pub email_verification_token_id: Uuid,
    pub user_id: Uuid,
    pub email_verification_token: Uuid,
    pub email_verification_token_expires_at: DateTime<Utc>,
    pub email_verification_token_created_at: DateTime<Utc>,
    pub email_verification_token_used_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = email_verification_tokens)]
pub struct NewEmailVerificationToken {
    pub user_id: Uuid,
    pub email_verification_token: Uuid,
    pub email_verification_token_expires_at: DateTime<Utc>,
    pub email_verification_token_created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = password_reset_tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PasswordResetToken {
    pub password_reset_token_id: Uuid,
    pub user_id: Uuid,
    pub password_reset_token: Uuid,
    pub password_reset_token_expires_at: DateTime<Utc>,
    pub password_reset_token_created_at: DateTime<Utc>,
    pub password_reset_token_used_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = password_reset_tokens)]
pub struct NewPasswordResetToken {
    pub user_id: Uuid,
    pub password_reset_token: Uuid,
    pub password_reset_token_expires_at: DateTime<Utc>,
    pub password_reset_token_created_at: DateTime<Utc>,
}
