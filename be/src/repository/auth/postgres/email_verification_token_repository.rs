use chrono::{DateTime, Utc};
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, SelectableHelper};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::{
    domain::auth::user::{EmailVerificationToken, NewEmailVerificationToken},
    schema::email_verification_tokens,
};

pub async fn insert_token(
    conn: &mut AsyncPgConnection,
    new_token: NewEmailVerificationToken,
) -> Result<usize, diesel::result::Error> {
    match diesel::insert_into(email_verification_tokens::table)
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
) -> Result<Option<EmailVerificationToken>, diesel::result::Error> {
    match email_verification_tokens::table
        .filter(email_verification_tokens::email_verification_token.eq(token))
        .for_update()
        .select(EmailVerificationToken::as_select())
        .first::<EmailVerificationToken>(conn)
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
        email_verification_tokens::table
            .filter(email_verification_tokens::email_verification_token_id.eq(token_id)),
    )
    .set(email_verification_tokens::email_verification_token_used_at.eq(now))
    .execute(conn)
    .await
    {
        Ok(rows) => Ok(rows),
        Err(error) => Err(error),
    }
}
