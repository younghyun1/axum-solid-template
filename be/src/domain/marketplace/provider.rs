use chrono::{DateTime, Utc};
use diesel::{Insertable, Queryable, Selectable};
use uuid::Uuid;

use crate::schema::{provider_blog_posts, provider_profiles, user_profile_extensions};

use super::enums::{BlogPostStatus, ModerationStatus, ProviderProfileStatus};

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = user_profile_extensions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserProfileExtension {
    pub user_profile_extension_id: Uuid,
    pub user_id: Uuid,
    pub user_profile_extension_display_name: Option<String>,
    pub user_profile_extension_bio: Option<String>,
    pub user_profile_extension_phone: Option<String>,
    pub user_profile_extension_public_email: Option<String>,
    pub user_profile_extension_created_at: DateTime<Utc>,
    pub user_profile_extension_updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = user_profile_extensions)]
pub struct NewUserProfileExtension {
    pub user_id: Uuid,
    pub user_profile_extension_display_name: Option<String>,
    pub user_profile_extension_bio: Option<String>,
    pub user_profile_extension_phone: Option<String>,
    pub user_profile_extension_public_email: Option<String>,
}

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = provider_profiles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProviderProfile {
    pub provider_profile_id: Uuid,
    pub user_id: Uuid,
    pub provider_profile_slug: String,
    pub provider_profile_display_name: String,
    pub provider_profile_headline: Option<String>,
    pub provider_profile_bio: Option<String>,
    pub provider_profile_service_area: Option<String>,
    pub provider_profile_status: ProviderProfileStatus,
    pub provider_profile_moderation_status: ModerationStatus,
    pub provider_profile_created_at: DateTime<Utc>,
    pub provider_profile_updated_at: DateTime<Utc>,
    pub provider_profile_primary_image_id: Option<Uuid>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = provider_profiles)]
pub struct NewProviderProfile {
    pub user_id: Uuid,
    pub provider_profile_slug: String,
    pub provider_profile_display_name: String,
    pub provider_profile_headline: Option<String>,
    pub provider_profile_bio: Option<String>,
    pub provider_profile_service_area: Option<String>,
    pub provider_profile_status: ProviderProfileStatus,
    pub provider_profile_moderation_status: ModerationStatus,
}

#[derive(Debug, Clone)]
pub struct ProviderProfileUpdate {
    pub provider_profile_slug: String,
    pub provider_profile_display_name: String,
    pub provider_profile_headline: Option<String>,
    pub provider_profile_bio: Option<String>,
    pub provider_profile_service_area: Option<String>,
    pub provider_profile_status: ProviderProfileStatus,
    pub provider_profile_updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = provider_blog_posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProviderBlogPost {
    pub provider_blog_post_id: Uuid,
    pub provider_profile_id: Uuid,
    pub provider_blog_post_slug: String,
    pub provider_blog_post_title: String,
    pub provider_blog_post_excerpt: Option<String>,
    pub provider_blog_post_body: String,
    pub provider_blog_post_status: BlogPostStatus,
    pub provider_blog_post_moderation_status: ModerationStatus,
    pub provider_blog_post_published_at: Option<DateTime<Utc>>,
    pub provider_blog_post_created_at: DateTime<Utc>,
    pub provider_blog_post_updated_at: DateTime<Utc>,
    pub provider_blog_post_hero_image_id: Option<Uuid>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = provider_blog_posts)]
pub struct NewProviderBlogPost {
    pub provider_profile_id: Uuid,
    pub provider_blog_post_slug: String,
    pub provider_blog_post_title: String,
    pub provider_blog_post_excerpt: Option<String>,
    pub provider_blog_post_body: String,
    pub provider_blog_post_status: BlogPostStatus,
    pub provider_blog_post_moderation_status: ModerationStatus,
    pub provider_blog_post_published_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct ProviderBlogPostUpdate {
    pub provider_blog_post_slug: String,
    pub provider_blog_post_title: String,
    pub provider_blog_post_excerpt: Option<String>,
    pub provider_blog_post_body: String,
    pub provider_blog_post_status: BlogPostStatus,
    pub provider_blog_post_moderation_status: ModerationStatus,
    pub provider_blog_post_published_at: Option<DateTime<Utc>>,
    pub provider_blog_post_updated_at: DateTime<Utc>,
}
