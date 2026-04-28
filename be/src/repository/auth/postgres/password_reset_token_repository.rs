use chrono::{DateTime, Utc};
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, SelectableHelper};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::{
    domain::auth::user::{NewPasswordResetToken, PasswordResetToken},
    schema::password_reset_tokens,
};

pub async fn insert_token(
    conn: &mut AsyncPgConnection,
    new_token: NewPasswordResetToken,
) -> Result<usize, diesel::result::Error> {
    match diesel::insert_into(password_reset_tokens::table)
        .values(new_token)
        .execute(conn)
        .await
    {
        Ok(rows) => Ok(rows),
        Err(error) => Err(error),
    }
}

pub async fn find_by_token(
    conn: &mut AsyncPgConnection,
    token: Uuid,
) -> Result<Option<PasswordResetToken>, diesel::result::Error> {
    match password_reset_tokens::table
        .filter(password_reset_tokens::password_reset_token.eq(token))
        .for_update()
        .select(PasswordResetToken::as_select())
        .first::<PasswordResetToken>(conn)
        .await
        .optional()
    {
        Ok(token) => Ok(token),
        Err(error) => Err(error),
    }
}

pub async fn mark_used(
    conn: &mut AsyncPgConnection,
    token_id: Uuid,
    now: DateTime<Utc>,
) -> Result<usize, diesel::result::Error> {
    match diesel::update(
        password_reset_tokens::table
            .filter(password_reset_tokens::password_reset_token_id.eq(token_id)),
    )
    .set(password_reset_tokens::password_reset_token_used_at.eq(now))
    .execute(conn)
    .await
    {
        Ok(rows) => Ok(rows),
        Err(error) => Err(error),
    }
}
