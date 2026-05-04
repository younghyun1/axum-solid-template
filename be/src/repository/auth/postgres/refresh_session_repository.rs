use chrono::{DateTime, Utc};
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, SelectableHelper};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::{
    domain::auth::refresh_session::{AuthRefreshSession, NewAuthRefreshSession},
    schema::auth_refresh_sessions,
};

pub async fn insert_session(
    conn: &mut AsyncPgConnection,
    new_session: NewAuthRefreshSession,
) -> Result<AuthRefreshSession, diesel::result::Error> {
    match diesel::insert_into(auth_refresh_sessions::table)
        .values(new_session)
        .returning(AuthRefreshSession::as_returning())
        .get_result::<AuthRefreshSession>(conn)
        .await
    {
        Ok(session) => Ok(session),
        Err(error) => Err(error),
    }
}

pub async fn find_by_token_hash(
    conn: &mut AsyncPgConnection,
    token_hash: &str,
) -> Result<Option<AuthRefreshSession>, diesel::result::Error> {
    match auth_refresh_sessions::table
        .filter(auth_refresh_sessions::auth_refresh_session_token_hash.eq(token_hash))
        .select(AuthRefreshSession::as_select())
        .first::<AuthRefreshSession>(conn)
        .await
        .optional()
    {
        Ok(session) => Ok(session),
        Err(error) => Err(error),
    }
}

pub async fn rotate_active_session(
    conn: &mut AsyncPgConnection,
    refresh_session_id: Uuid,
    now: DateTime<Utc>,
) -> Result<usize, diesel::result::Error> {
    match diesel::update(
        auth_refresh_sessions::table
            .filter(auth_refresh_sessions::auth_refresh_session_id.eq(refresh_session_id))
            .filter(auth_refresh_sessions::auth_refresh_session_revoked_at.is_null()),
    )
    .set((
        auth_refresh_sessions::auth_refresh_session_last_used_at.eq(now),
        auth_refresh_sessions::auth_refresh_session_rotated_at.eq(now),
        auth_refresh_sessions::auth_refresh_session_revoked_at.eq(now),
    ))
    .execute(conn)
    .await
    {
        Ok(rows) => Ok(rows),
        Err(error) => Err(error),
    }
}

pub async fn revoke_by_token_hash(
    conn: &mut AsyncPgConnection,
    token_hash: &str,
    now: DateTime<Utc>,
) -> Result<usize, diesel::result::Error> {
    match diesel::update(
        auth_refresh_sessions::table
            .filter(auth_refresh_sessions::auth_refresh_session_token_hash.eq(token_hash))
            .filter(auth_refresh_sessions::auth_refresh_session_revoked_at.is_null()),
    )
    .set(auth_refresh_sessions::auth_refresh_session_revoked_at.eq(now))
    .execute(conn)
    .await
    {
        Ok(rows) => Ok(rows),
        Err(error) => Err(error),
    }
}
