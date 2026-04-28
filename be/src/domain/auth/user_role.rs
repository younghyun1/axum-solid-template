use diesel::{Insertable, Queryable, Selectable};
use uuid::Uuid;

use crate::schema::user_roles;

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = user_roles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserRole {
    pub user_role_id: Uuid,
    pub user_id: Uuid,
    pub role_id: Uuid,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = user_roles)]
pub struct NewUserRole {
    pub user_id: Uuid,
    pub role_id: Uuid,
}
