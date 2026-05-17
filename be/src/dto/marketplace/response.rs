use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::domain::marketplace::enums::{
    BanScope, BannerPlacement, BannerStatus, BlogPostStatus, ImageType, ImageUploadStatus,
    ImageVisibility, ModerationStatus, PaymentIntentStatus, PaymentProvider,
    PaymentTransactionKind, PaymentTransactionStatus, ProviderProfileStatus,
};
use crate::domain::marketplace::search::MarketplaceSearchResultKind;

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct ImageResponse {
    pub image_id: Uuid,
    pub image_type: ImageType,
    pub upload_status: ImageUploadStatus,
    pub visibility: ImageVisibility,
    pub bucket: String,
    pub object_key: String,
    pub public_url: Option<String>,
    pub mime_type: String,
    pub byte_size: i64,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub uploaded_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct UserProfileResponse {
    pub user_profile_extension_id: Uuid,
    pub user_id: Uuid,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub phone: Option<String>,
    pub public_email: Option<String>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct ProviderProfileResponse {
    pub provider_profile_id: Uuid,
    pub user_id: Uuid,
    pub slug: String,
    pub display_name: String,
    pub headline: Option<String>,
    pub bio: Option<String>,
    pub subdivision: Option<ProviderSubdivisionResponse>,
    pub status: ProviderProfileStatus,
    pub moderation_status: ModerationStatus,
    pub primary_image_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct ProviderDirectoryCardResponse {
    pub provider_profile_id: Uuid,
    pub slug: String,
    pub display_name: String,
    pub headline: Option<String>,
    pub subdivision: Option<ProviderSubdivisionResponse>,
    pub primary_image: Option<ImageResponse>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct ProviderSubdivisionResponse {
    pub subdivision_id: i32,
    pub country_code: i32,
    pub country_alpha2: String,
    pub subdivision_code: String,
    pub subdivision_name: String,
    pub subdivision_type: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct ProviderDirectoryResponse {
    pub providers: Vec<ProviderDirectoryCardResponse>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct MarketplaceSearchResultResponse {
    pub kind: MarketplaceSearchResultKind,
    pub title: String,
    pub subtitle: String,
    pub slug: String,
    pub url_path: String,
    pub snippet: String,
    pub score: f32,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct MarketplaceSearchResponse {
    pub results: Vec<MarketplaceSearchResultResponse>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct MarketplaceSearchReindexResponse {
    pub indexed_documents: usize,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct MarketplaceCacheClearResponse {
    pub cleared: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct ProviderBlogPostResponse {
    pub provider_blog_post_id: Uuid,
    pub provider_profile_id: Uuid,
    pub slug: String,
    pub title: String,
    pub excerpt: Option<String>,
    pub body: Option<String>,
    pub rendered_html: Option<String>,
    pub status: BlogPostStatus,
    pub moderation_status: ModerationStatus,
    pub hero_image_id: Option<Uuid>,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct ProviderDetailResponse {
    pub profile: ProviderProfileResponse,
    pub images: Vec<ImageResponse>,
    pub blog_posts: Vec<ProviderBlogPostResponse>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct PaymentIntentResponse {
    pub payment_intent_id: Uuid,
    pub user_id: Uuid,
    pub provider_profile_id: Uuid,
    pub amount_minor_units: i64,
    pub currency_code: i32,
    pub payment_provider: PaymentProvider,
    pub status: PaymentIntentStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct PaymentTransactionResponse {
    pub payment_transaction_id: Uuid,
    pub payment_intent_id: Uuid,
    pub kind: PaymentTransactionKind,
    pub status: PaymentTransactionStatus,
    pub amount_minor_units: i64,
    pub currency_code: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct PaymentIntentListResponse {
    pub payment_intents: Vec<PaymentIntentResponse>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct BanResponse {
    pub moderation_ban_id: Uuid,
    pub target_user_id: Uuid,
    pub actor_user_id: Uuid,
    pub scope: BanScope,
    pub reason: String,
    pub starts_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub revoked_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct BanListResponse {
    pub bans: Vec<BanResponse>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct CentralBlogPostResponse {
    pub central_blog_post_id: Uuid,
    pub author_user_id: Uuid,
    pub slug: String,
    pub title: String,
    pub excerpt: Option<String>,
    pub body: String,
    pub rendered_html: String,
    pub status: BlogPostStatus,
    pub moderation_status: ModerationStatus,
    pub published_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct BannerResponse {
    pub advertisement_banner_id: Uuid,
    pub placement: BannerPlacement,
    pub status: BannerStatus,
    pub title: String,
    pub target_url: String,
    pub priority: i32,
    pub starts_at: DateTime<Utc>,
    pub ends_at: Option<DateTime<Utc>>,
    pub image_id: Option<Uuid>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct BannerListResponse {
    pub banners: Vec<BannerResponse>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct AdminOverviewResponse {
    pub provider_count: i64,
    pub active_ban_count: i64,
    pub payment_intent_count: i64,
    pub active_banner_count: i64,
}
