use diesel::{Queryable, Selectable};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::schema::roles;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum RoleType {
    Younghyun = 0,
    Moderator = 1,
    User = 2,
    Guest = 3,
}

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = roles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Role {
    pub role_id: Uuid,
    pub role_name: String,
    pub role_description: Option<String>,
}

impl RoleType {
    pub fn from_uuid(role_id: Uuid) -> Option<Self> {
        match role_id.as_u128() {
            ROLE_YOUNGHYUN => Some(Self::Younghyun),
            ROLE_MODERATOR => Some(Self::Moderator),
            ROLE_USER => Some(Self::User),
            ROLE_GUEST => Some(Self::Guest),
            _ => None,
        }
    }

    pub fn id(self) -> Uuid {
        match self {
            Self::Younghyun => Uuid::from_u128(ROLE_YOUNGHYUN),
            Self::Moderator => Uuid::from_u128(ROLE_MODERATOR),
            Self::User => Uuid::from_u128(ROLE_USER),
            Self::Guest => Uuid::from_u128(ROLE_GUEST),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Younghyun => "younghyun",
            Self::Moderator => "moderator",
            Self::User => "user",
            Self::Guest => "guest",
        }
    }

    pub fn is_superuser(self) -> bool {
        match self {
            Self::Younghyun => true,
            Self::Moderator | Self::User | Self::Guest => false,
        }
    }

    pub fn access_level(self) -> u8 {
        match self {
            Self::Younghyun => 3,
            Self::Moderator => 2,
            Self::User => 1,
            Self::Guest => 0,
        }
    }
}

impl Role {
    pub fn role_type(&self) -> Option<RoleType> {
        RoleType::from_uuid(self.role_id)
    }
}

const ROLE_YOUNGHYUN: u128 = 2131042872073453539493660941469037155;
const ROLE_MODERATOR: u128 = 2131042883709330333470894399469323316;
const ROLE_USER: u128 = 2131042888123140653623930835701279230;
const ROLE_GUEST: u128 = 2131042895169936790354381715792830592;
