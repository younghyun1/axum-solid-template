use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Debug, Clone, Copy, Deserialize, Serialize, ToSchema, diesel::AsExpression, diesel::FromSqlRow,
)]
#[diesel(sql_type = crate::schema::sql_types::ImageType)]
#[serde(rename_all = "snake_case")]
pub enum ImageType {
    UserProfile,
    ProviderProfile,
    ProviderBlog,
    CentralBlog,
    AdvertisementBanner,
}

#[derive(
    Debug, Clone, Copy, Deserialize, Serialize, ToSchema, diesel::AsExpression, diesel::FromSqlRow,
)]
#[diesel(sql_type = crate::schema::sql_types::ImageUploadStatus)]
#[serde(rename_all = "snake_case")]
pub enum ImageUploadStatus {
    Pending,
    Uploaded,
    Failed,
}

#[derive(
    Debug, Clone, Copy, Deserialize, Serialize, ToSchema, diesel::AsExpression, diesel::FromSqlRow,
)]
#[diesel(sql_type = crate::schema::sql_types::ImageVisibility)]
#[serde(rename_all = "snake_case")]
pub enum ImageVisibility {
    Private,
    Public,
    Hidden,
}

#[derive(
    Debug, Clone, Copy, Deserialize, Serialize, ToSchema, diesel::AsExpression, diesel::FromSqlRow,
)]
#[diesel(sql_type = crate::schema::sql_types::ProviderProfileStatus)]
#[serde(rename_all = "snake_case")]
pub enum ProviderProfileStatus {
    Draft,
    Published,
    Suspended,
}

#[derive(
    Debug, Clone, Copy, Deserialize, Serialize, ToSchema, diesel::AsExpression, diesel::FromSqlRow,
)]
#[diesel(sql_type = crate::schema::sql_types::ModerationStatus)]
#[serde(rename_all = "snake_case")]
pub enum ModerationStatus {
    Pending,
    Approved,
    Rejected,
}

#[derive(
    Debug, Clone, Copy, Deserialize, Serialize, ToSchema, diesel::AsExpression, diesel::FromSqlRow,
)]
#[diesel(sql_type = crate::schema::sql_types::BlogPostStatus)]
#[serde(rename_all = "snake_case")]
pub enum BlogPostStatus {
    Draft,
    Published,
    Archived,
}

#[derive(
    Debug, Clone, Copy, Deserialize, Serialize, ToSchema, diesel::AsExpression, diesel::FromSqlRow,
)]
#[diesel(sql_type = crate::schema::sql_types::BanScope)]
#[serde(rename_all = "snake_case")]
pub enum BanScope {
    Account,
    Provider,
    Content,
}

#[derive(
    Debug, Clone, Copy, Deserialize, Serialize, ToSchema, diesel::AsExpression, diesel::FromSqlRow,
)]
#[diesel(sql_type = crate::schema::sql_types::PaymentProvider)]
#[serde(rename_all = "snake_case")]
pub enum PaymentProvider {
    Manual,
    External,
}

#[derive(
    Debug, Clone, Copy, Deserialize, Serialize, ToSchema, diesel::AsExpression, diesel::FromSqlRow,
)]
#[diesel(sql_type = crate::schema::sql_types::PaymentIntentStatus)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentStatus {
    Created,
    RequiresAction,
    Authorized,
    Captured,
    Cancelled,
    Failed,
    Refunded,
}

#[derive(
    Debug, Clone, Copy, Deserialize, Serialize, ToSchema, diesel::AsExpression, diesel::FromSqlRow,
)]
#[diesel(sql_type = crate::schema::sql_types::PaymentTransactionKind)]
#[serde(rename_all = "snake_case")]
pub enum PaymentTransactionKind {
    Authorization,
    Capture,
    Refund,
    Adjustment,
}

#[derive(
    Debug, Clone, Copy, Deserialize, Serialize, ToSchema, diesel::AsExpression, diesel::FromSqlRow,
)]
#[diesel(sql_type = crate::schema::sql_types::PaymentTransactionStatus)]
#[serde(rename_all = "snake_case")]
pub enum PaymentTransactionStatus {
    Pending,
    Succeeded,
    Failed,
}

#[derive(
    Debug, Clone, Copy, Deserialize, Serialize, ToSchema, diesel::AsExpression, diesel::FromSqlRow,
)]
#[diesel(sql_type = crate::schema::sql_types::ProcessorEventStatus)]
#[serde(rename_all = "snake_case")]
pub enum ProcessorEventStatus {
    Pending,
    Processed,
    Failed,
}

#[derive(
    Debug, Clone, Copy, Deserialize, Serialize, ToSchema, diesel::AsExpression, diesel::FromSqlRow,
)]
#[diesel(sql_type = crate::schema::sql_types::BannerPlacement)]
#[serde(rename_all = "snake_case")]
pub enum BannerPlacement {
    HomepageTop,
    DirectorySidebar,
    ProviderProfile,
}

#[derive(
    Debug, Clone, Copy, Deserialize, Serialize, ToSchema, diesel::AsExpression, diesel::FromSqlRow,
)]
#[diesel(sql_type = crate::schema::sql_types::BannerStatus)]
#[serde(rename_all = "snake_case")]
pub enum BannerStatus {
    Draft,
    Active,
    Paused,
    Archived,
}
