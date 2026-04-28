use diesel::{Queryable, Selectable};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::schema::roles;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum RoleType {
    Admin = 0,
    Moderator = 1,
    ServiceProvider = 2,
    User = 3,
    Guest = 4,
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
            ROLE_ADMIN => Some(Self::Admin),
            ROLE_MODERATOR => Some(Self::Moderator),
            ROLE_SERVICE_PROVIDER => Some(Self::ServiceProvider),
            ROLE_USER => Some(Self::User),
            ROLE_GUEST => Some(Self::Guest),
            _ => None,
        }
    }

    pub fn id(self) -> Uuid {
        match self {
            Self::Admin => Uuid::from_u128(ROLE_ADMIN),
            Self::Moderator => Uuid::from_u128(ROLE_MODERATOR),
            Self::ServiceProvider => Uuid::from_u128(ROLE_SERVICE_PROVIDER),
            Self::User => Uuid::from_u128(ROLE_USER),
            Self::Guest => Uuid::from_u128(ROLE_GUEST),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Admin => "admin",
            Self::Moderator => "moderator",
            Self::ServiceProvider => "service_provider",
            Self::User => "user",
            Self::Guest => "guest",
        }
    }

    pub fn is_admin(self) -> bool {
        match self {
            Self::Admin => true,
            Self::Moderator | Self::ServiceProvider | Self::User | Self::Guest => false,
        }
    }

    pub fn access_level(self) -> u8 {
        match self {
            Self::Admin => 4,
            Self::Moderator => 3,
            Self::ServiceProvider => 2,
            Self::User => 1,
            Self::Guest => 0,
        }
    }

    pub fn is_moderator(self) -> bool {
        match self {
            Self::Moderator => true,
            Self::Admin | Self::ServiceProvider | Self::User | Self::Guest => false,
        }
    }

    pub fn is_service_provider(self) -> bool {
        match self {
            Self::ServiceProvider => true,
            Self::Admin | Self::Moderator | Self::User | Self::Guest => false,
        }
    }

    pub fn is_user_client(self) -> bool {
        match self {
            Self::User => true,
            Self::Admin | Self::Moderator | Self::ServiceProvider | Self::Guest => false,
        }
    }

    pub fn is_guest(self) -> bool {
        match self {
            Self::Guest => true,
            Self::Admin | Self::Moderator | Self::ServiceProvider | Self::User => false,
        }
    }

    pub fn has_min_access_level(self, minimum_role: Self) -> bool {
        self.access_level() >= minimum_role.access_level()
    }
}

impl Role {
    pub fn role_type(&self) -> Option<RoleType> {
        RoleType::from_uuid(self.role_id)
    }
}

const ROLE_ADMIN: u128 = 2131042872073453539493660941469037155;
const ROLE_MODERATOR: u128 = 2131042883709330333470894399469323316;
const ROLE_SERVICE_PROVIDER: u128 = 2148683422655221045441942470519118958;
const ROLE_USER: u128 = 2131042888123140653623930835701279230;
const ROLE_GUEST: u128 = 2131042895169936790354381715792830592;

#[cfg(test)]
mod tests {
    use super::RoleType;

    #[test]
    fn role_ids_round_trip_to_canonical_types() {
        let roles = [
            RoleType::Admin,
            RoleType::Moderator,
            RoleType::ServiceProvider,
            RoleType::User,
            RoleType::Guest,
        ];

        for role in roles {
            assert_eq!(RoleType::from_uuid(role.id()), Some(role));
        }
    }

    #[test]
    fn role_names_are_service_board_canonical_names() {
        assert_eq!(RoleType::Admin.as_str(), "admin");
        assert_eq!(RoleType::Moderator.as_str(), "moderator");
        assert_eq!(RoleType::ServiceProvider.as_str(), "service_provider");
        assert_eq!(RoleType::User.as_str(), "user");
        assert_eq!(RoleType::Guest.as_str(), "guest");
    }

    #[test]
    fn role_access_levels_order_service_provider_between_user_and_moderator() {
        assert!(RoleType::ServiceProvider.access_level() > RoleType::User.access_level());
        assert!(RoleType::ServiceProvider.access_level() < RoleType::Moderator.access_level());
        assert!(RoleType::Admin.access_level() > RoleType::Moderator.access_level());
    }

    #[test]
    fn role_helpers_distinguish_exact_roles_and_minimum_access() {
        assert!(RoleType::Admin.is_admin());
        assert!(RoleType::Moderator.is_moderator());
        assert!(RoleType::ServiceProvider.is_service_provider());
        assert!(RoleType::User.is_user_client());
        assert!(RoleType::Guest.is_guest());
        assert!(RoleType::Admin.has_min_access_level(RoleType::ServiceProvider));
        assert!(!RoleType::User.has_min_access_level(RoleType::ServiceProvider));
    }
}
