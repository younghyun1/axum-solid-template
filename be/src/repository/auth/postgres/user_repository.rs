use std::fmt;

use chrono::{DateTime, Utc};
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, SelectableHelper};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::{
    domain::auth::user::{NewUser, User, UserInfo},
    schema::users,
};

#[derive(Debug)]
pub enum InsertUserError {
    UniqueViolation,
    Database { error: diesel::result::Error },
}

impl fmt::Display for InsertUserError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UniqueViolation => formatter.write_str("user unique constraint violation"),
            Self::Database { error } => write!(formatter, "failed to insert user: {error}"),
        }
    }
}

pub async fn insert_user(
    conn: &mut AsyncPgConnection,
    new_user: NewUser,
) -> Result<User, InsertUserError> {
    match diesel::insert_into(users::table)
        .values(new_user)
        .returning(User::as_returning())
        .get_result::<User>(conn)
        .await
    {
        Ok(user) => Ok(user),
        Err(diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::UniqueViolation,
            _,
        )) => Err(InsertUserError::UniqueViolation),
        Err(error) => Err(InsertUserError::Database { error }),
    }
}

pub async fn find_user_by_id(
    conn: &mut AsyncPgConnection,
    user_id: Uuid,
) -> Result<Option<User>, diesel::result::Error> {
    match users::table
        .filter(users::user_id.eq(user_id))
        .select(User::as_select())
        .first::<User>(conn)
        .await
        .optional()
    {
        Ok(user) => Ok(user),
        Err(error) => Err(error),
    }
}

pub async fn find_user_info_by_id(
    conn: &mut AsyncPgConnection,
    user_id: Uuid,
) -> Result<Option<UserInfo>, diesel::result::Error> {
    match users::table
        .filter(users::user_id.eq(user_id))
        .select(UserInfo::as_select())
        .first::<UserInfo>(conn)
        .await
        .optional()
    {
        Ok(user_info) => Ok(user_info),
        Err(error) => Err(error),
    }
}

pub async fn find_user_by_email(
    conn: &mut AsyncPgConnection,
    user_email: &str,
) -> Result<Option<User>, diesel::result::Error> {
    match users::table
        .filter(users::user_email.eq(user_email))
        .select(User::as_select())
        .first::<User>(conn)
        .await
        .optional()
    {
        Ok(user) => Ok(user),
        Err(error) => Err(error),
    }
}

pub async fn find_user_by_name(
    conn: &mut AsyncPgConnection,
    user_name: &str,
) -> Result<Option<User>, diesel::result::Error> {
    match users::table
        .filter(users::user_name.eq(user_name))
        .select(User::as_select())
        .first::<User>(conn)
        .await
        .optional()
    {
        Ok(user) => Ok(user),
        Err(error) => Err(error),
    }
}

pub async fn find_user_id_by_email(
    conn: &mut AsyncPgConnection,
    user_email: &str,
) -> Result<Option<Uuid>, diesel::result::Error> {
    match users::table
        .filter(users::user_email.eq(user_email))
        .select(users::user_id)
        .first::<Uuid>(conn)
        .await
        .optional()
    {
        Ok(user_id) => Ok(user_id),
        Err(error) => Err(error),
    }
}

pub async fn update_last_login(
    conn: &mut AsyncPgConnection,
    user_id: Uuid,
    now: DateTime<Utc>,
) -> Result<usize, diesel::result::Error> {
    match diesel::update(users::table.filter(users::user_id.eq(user_id)))
        .set((
            users::user_last_login_at.eq(now),
            users::user_updated_at.eq(now),
        ))
        .execute(conn)
        .await
    {
        Ok(rows) => Ok(rows),
        Err(error) => Err(error),
    }
}

pub async fn update_password_after_reset(
    conn: &mut AsyncPgConnection,
    user_id: Uuid,
    password_hash: String,
    now: DateTime<Utc>,
) -> Result<User, diesel::result::Error> {
    match diesel::update(users::table.filter(users::user_id.eq(user_id)))
        .set((
            users::user_password_hash.eq(password_hash),
            users::user_password_changed_at.eq(now),
            users::user_updated_at.eq(now),
            users::user_auth_token_version.eq(users::user_auth_token_version + 1),
        ))
        .returning(User::as_returning())
        .get_result::<User>(conn)
        .await
    {
        Ok(user) => Ok(user),
        Err(error) => Err(error),
    }
}

pub async fn mark_email_verified(
    conn: &mut AsyncPgConnection,
    user_id: Uuid,
    now: DateTime<Utc>,
) -> Result<User, diesel::result::Error> {
    match diesel::update(users::table.filter(users::user_id.eq(user_id)))
        .set((
            users::user_is_email_verified.eq(true),
            users::user_updated_at.eq(now),
        ))
        .returning(User::as_returning())
        .get_result::<User>(conn)
        .await
    {
        Ok(user) => Ok(user),
        Err(error) => Err(error),
    }
}
