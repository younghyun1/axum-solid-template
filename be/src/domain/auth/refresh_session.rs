use chrono::{DateTime, Utc};
use diesel::{Insertable, Queryable, Selectable};
use uuid::Uuid;

use crate::schema::auth_refresh_sessions;

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = auth_refresh_sessions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AuthRefreshSession {
    pub auth_refresh_session_id: Uuid,
    pub user_id: Uuid,
    pub auth_refresh_session_token_hash: String,
    pub auth_refresh_session_created_at: DateTime<Utc>,
    pub auth_refresh_session_expires_at: DateTime<Utc>,
    pub auth_refresh_session_last_used_at: Option<DateTime<Utc>>,
    pub auth_refresh_session_rotated_at: Option<DateTime<Utc>>,
    pub auth_refresh_session_revoked_at: Option<DateTime<Utc>>,
    pub auth_refresh_session_user_auth_token_version: i32,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = auth_refresh_sessions)]
pub struct NewAuthRefreshSession {
    pub user_id: Uuid,
    pub auth_refresh_session_token_hash: String,
    pub auth_refresh_session_created_at: DateTime<Utc>,
    pub auth_refresh_session_expires_at: DateTime<Utc>,
    pub auth_refresh_session_user_auth_token_version: i32,
}
