use chrono::{DateTime, Utc};
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::domain::marketplace::enums::{
    BanScope, BannerPlacement, BannerStatus, BlogPostStatus, ImageType, ImageVisibility,
    PaymentProvider, ProviderProfileStatus,
};

#[derive(Debug, Deserialize, IntoParams)]
pub struct ProviderDirectoryQuery {
    pub q: Option<String>,
    pub service_area: Option<String>,
    pub limit: Option<i64>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpsertUserProfileRequest {
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub phone: Option<String>,
    pub public_email: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpsertProviderProfileRequest {
    pub slug: Option<String>,
    pub display_name: String,
    pub headline: Option<String>,
    pub bio: Option<String>,
    pub service_area: Option<String>,
    pub status: ProviderProfileStatus,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateProviderBlogPostRequest {
    pub slug: Option<String>,
    pub title: String,
    pub excerpt: Option<String>,
    pub body: String,
    pub status: BlogPostStatus,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateImageRequest {
    pub image_type: ImageType,
    pub visibility: ImageVisibility,
    pub bucket: String,
    pub object_key: String,
    pub public_url: Option<String>,
    pub mime_type: String,
    pub byte_size: i64,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub checksum_sha256: Option<String>,
    pub provider_blog_post_id: Option<Uuid>,
    pub central_blog_post_id: Option<Uuid>,
    pub advertisement_banner_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CompleteImageUploadRequest {
    pub public_url: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreatePaymentIntentRequest {
    pub provider_profile_id: Uuid,
    pub amount_minor_units: i64,
    pub currency_code: i32,
    pub payment_provider: PaymentProvider,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateBanRequest {
    pub target_user_id: Uuid,
    pub scope: BanScope,
    pub reason: String,
    pub starts_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateCentralBlogPostRequest {
    pub slug: Option<String>,
    pub title: String,
    pub excerpt: Option<String>,
    pub body: String,
    pub status: BlogPostStatus,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateBannerRequest {
    pub placement: BannerPlacement,
    pub status: BannerStatus,
    pub title: String,
    pub target_url: String,
    pub priority: i32,
    pub starts_at: DateTime<Utc>,
    pub ends_at: Option<DateTime<Utc>>,
}
