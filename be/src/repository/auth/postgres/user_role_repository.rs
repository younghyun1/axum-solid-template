use diesel::{ExpressionMethods, OptionalExtension, QueryDsl};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::{
    domain::auth::{role::RoleType, user_role::NewUserRole},
    schema::user_roles,
};

pub async fn role_for_user(
    conn: &mut AsyncPgConnection,
    user_id: Uuid,
) -> Result<RoleType, diesel::result::Error> {
    let role_id_result = user_roles::table
        .filter(user_roles::user_id.eq(user_id))
        .select(user_roles::role_id)
        .first::<Uuid>(conn)
        .await
        .optional();

    let role_id = match role_id_result {
        Ok(Some(role_id)) => role_id,
        Ok(None) => return Ok(RoleType::User),
        Err(error) => return Err(error),
    };

    match RoleType::from_uuid(role_id) {
        Some(role_type) => Ok(role_type),
        None => Ok(RoleType::User),
    }
}

pub async fn insert_for_user(
    conn: &mut AsyncPgConnection,
    user_id: Uuid,
    role_type: RoleType,
) -> Result<(), diesel::result::Error> {
    let new_user_role = NewUserRole {
        user_id,
        role_id: role_type.id(),
    };

    match diesel::insert_into(user_roles::table)
        .values(new_user_role)
        .execute(conn)
        .await
    {
        Ok(_) => Ok(()),
        Err(error) => Err(error),
    }
}
