use std::{error::Error, fmt, io::Write};

use diesel::{
    deserialize::{self, FromSql},
    pg::{Pg, PgValue},
    serialize::{self, IsNull, Output, ToSql},
};

use super::enums::{
    BanScope, BannerPlacement, BannerStatus, BlogPostStatus, ImageType, ImageUploadStatus,
    ImageVisibility, ModerationStatus, PaymentIntentStatus, PaymentProvider,
    PaymentTransactionKind, PaymentTransactionStatus, ProcessorEventStatus, ProviderProfileStatus,
};

#[derive(Debug, Clone)]
pub struct EnumParseError {
    enum_name: &'static str,
    value: String,
}

impl fmt::Display for EnumParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "invalid {} value: {}",
            self.enum_name, self.value
        )
    }
}

impl Error for EnumParseError {}

macro_rules! pg_enum_impl {
    ($enum_type:ident, $sql_type:path, {$($variant:ident => $value:literal),+ $(,)?}) => {
        impl $enum_type {
            pub fn as_str(self) -> &'static str {
                match self {
                    $(Self::$variant => $value,)+
                }
            }
        }

        impl TryFrom<&str> for $enum_type {
            type Error = EnumParseError;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                match value {
                    $($value => Ok(Self::$variant),)+
                    _ => Err(EnumParseError {
                        enum_name: stringify!($enum_type),
                        value: value.to_string(),
                    }),
                }
            }
        }

        impl FromSql<$sql_type, Pg> for $enum_type {
            fn from_sql(value: PgValue<'_>) -> deserialize::Result<Self> {
                let value = match std::str::from_utf8(value.as_bytes()) {
                    Ok(value) => value,
                    Err(error) => return Err(Box::new(error)),
                };
                match Self::try_from(value) {
                    Ok(parsed) => Ok(parsed),
                    Err(error) => Err(Box::new(error)),
                }
            }
        }

        impl ToSql<$sql_type, Pg> for $enum_type {
            fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
                match out.write_all(self.as_str().as_bytes()) {
                    Ok(()) => Ok(IsNull::No),
                    Err(error) => Err(Box::new(error)),
                }
            }
        }
    };
}

pg_enum_impl!(ImageType, crate::schema::sql_types::ImageType, {
    UserProfile => "user_profile",
    ProviderProfile => "provider_profile",
    ProviderBlog => "provider_blog",
    CentralBlog => "central_blog",
    AdvertisementBanner => "advertisement_banner",
});

pg_enum_impl!(ImageUploadStatus, crate::schema::sql_types::ImageUploadStatus, {
    Pending => "pending",
    Uploaded => "uploaded",
    Failed => "failed",
});

pg_enum_impl!(ImageVisibility, crate::schema::sql_types::ImageVisibility, {
    Private => "private",
    Public => "public",
    Hidden => "hidden",
});

pg_enum_impl!(ProviderProfileStatus, crate::schema::sql_types::ProviderProfileStatus, {
    Draft => "draft",
    Published => "published",
    Suspended => "suspended",
});

pg_enum_impl!(ModerationStatus, crate::schema::sql_types::ModerationStatus, {
    Pending => "pending",
    Approved => "approved",
    Rejected => "rejected",
});

pg_enum_impl!(BlogPostStatus, crate::schema::sql_types::BlogPostStatus, {
    Draft => "draft",
    Published => "published",
    Archived => "archived",
});

pg_enum_impl!(BanScope, crate::schema::sql_types::BanScope, {
    Account => "account",
    Provider => "provider",
    Content => "content",
});

pg_enum_impl!(PaymentProvider, crate::schema::sql_types::PaymentProvider, {
    Manual => "manual",
    External => "external",
});

pg_enum_impl!(PaymentIntentStatus, crate::schema::sql_types::PaymentIntentStatus, {
    Created => "created",
    RequiresAction => "requires_action",
    Authorized => "authorized",
    Captured => "captured",
    Cancelled => "cancelled",
    Failed => "failed",
    Refunded => "refunded",
});

pg_enum_impl!(PaymentTransactionKind, crate::schema::sql_types::PaymentTransactionKind, {
    Authorization => "authorization",
    Capture => "capture",
    Refund => "refund",
    Adjustment => "adjustment",
});

pg_enum_impl!(PaymentTransactionStatus, crate::schema::sql_types::PaymentTransactionStatus, {
    Pending => "pending",
    Succeeded => "succeeded",
    Failed => "failed",
});

pg_enum_impl!(ProcessorEventStatus, crate::schema::sql_types::ProcessorEventStatus, {
    Pending => "pending",
    Processed => "processed",
    Failed => "failed",
});

pg_enum_impl!(BannerPlacement, crate::schema::sql_types::BannerPlacement, {
    HomepageTop => "homepage_top",
    DirectorySidebar => "directory_sidebar",
    ProviderProfile => "provider_profile",
});

pg_enum_impl!(BannerStatus, crate::schema::sql_types::BannerStatus, {
    Draft => "draft",
    Active => "active",
    Paused => "paused",
    Archived => "archived",
});
