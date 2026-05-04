use chrono::{DateTime, Utc};
use diesel::{Insertable, Queryable, Selectable};
use uuid::Uuid;

use crate::schema::{advertisement_banners, central_blog_posts, moderation_bans};

use super::enums::{BanScope, BannerPlacement, BannerStatus, BlogPostStatus, ModerationStatus};

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = moderation_bans)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ModerationBan {
    pub moderation_ban_id: Uuid,
    pub target_user_id: Uuid,
    pub actor_user_id: Uuid,
    pub revoked_by_user_id: Option<Uuid>,
    pub moderation_ban_scope: BanScope,
    pub moderation_ban_reason: String,
    pub moderation_ban_starts_at: DateTime<Utc>,
    pub moderation_ban_expires_at: Option<DateTime<Utc>>,
    pub moderation_ban_revoked_at: Option<DateTime<Utc>>,
    pub moderation_ban_created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = moderation_bans)]
pub struct NewModerationBan {
    pub target_user_id: Uuid,
    pub actor_user_id: Uuid,
    pub moderation_ban_scope: BanScope,
    pub moderation_ban_reason: String,
    pub moderation_ban_starts_at: DateTime<Utc>,
    pub moderation_ban_expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = central_blog_posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CentralBlogPost {
    pub central_blog_post_id: Uuid,
    pub author_user_id: Uuid,
    pub central_blog_post_slug: String,
    pub central_blog_post_title: String,
    pub central_blog_post_excerpt: Option<String>,
    pub central_blog_post_body: String,
    pub central_blog_post_status: BlogPostStatus,
    pub central_blog_post_moderation_status: ModerationStatus,
    pub central_blog_post_published_at: Option<DateTime<Utc>>,
    pub central_blog_post_created_at: DateTime<Utc>,
    pub central_blog_post_updated_at: DateTime<Utc>,
    pub central_blog_post_hero_image_id: Option<Uuid>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = central_blog_posts)]
pub struct NewCentralBlogPost {
    pub author_user_id: Uuid,
    pub central_blog_post_slug: String,
    pub central_blog_post_title: String,
    pub central_blog_post_excerpt: Option<String>,
    pub central_blog_post_body: String,
    pub central_blog_post_status: BlogPostStatus,
    pub central_blog_post_moderation_status: ModerationStatus,
    pub central_blog_post_published_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = advertisement_banners)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AdvertisementBanner {
    pub advertisement_banner_id: Uuid,
    pub created_by_user_id: Uuid,
    pub advertisement_banner_placement: BannerPlacement,
    pub advertisement_banner_status: BannerStatus,
    pub advertisement_banner_title: String,
    pub advertisement_banner_target_url: String,
    pub advertisement_banner_priority: i32,
    pub advertisement_banner_starts_at: DateTime<Utc>,
    pub advertisement_banner_ends_at: Option<DateTime<Utc>>,
    pub advertisement_banner_created_at: DateTime<Utc>,
    pub advertisement_banner_updated_at: DateTime<Utc>,
    pub advertisement_banner_image_id: Option<Uuid>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = advertisement_banners)]
pub struct NewAdvertisementBanner {
    pub created_by_user_id: Uuid,
    pub advertisement_banner_placement: BannerPlacement,
    pub advertisement_banner_status: BannerStatus,
    pub advertisement_banner_title: String,
    pub advertisement_banner_target_url: String,
    pub advertisement_banner_priority: i32,
    pub advertisement_banner_starts_at: DateTime<Utc>,
    pub advertisement_banner_ends_at: Option<DateTime<Utc>>,
}
