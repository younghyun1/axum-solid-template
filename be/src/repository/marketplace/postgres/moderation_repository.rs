use chrono::{DateTime, Utc};
use diesel::{ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::{
    domain::marketplace::{
        enums::ModerationStatus,
        provider::{ProviderBlogPost, ProviderBlogPostUpdate, ProviderProfile},
    },
    schema::{provider_blog_posts, provider_profiles},
};

pub async fn update_provider_profile_moderation(
    conn: &mut AsyncPgConnection,
    provider_profile_id: Uuid,
    moderation_status: ModerationStatus,
    now: DateTime<Utc>,
) -> Result<ProviderProfile, diesel::result::Error> {
    match diesel::update(
        provider_profiles::table
            .filter(provider_profiles::provider_profile_id.eq(provider_profile_id)),
    )
    .set((
        provider_profiles::provider_profile_moderation_status.eq(moderation_status),
        provider_profiles::provider_profile_updated_at.eq(now),
    ))
    .returning(ProviderProfile::as_returning())
    .get_result::<ProviderProfile>(conn)
    .await
    {
        Ok(profile) => Ok(profile),
        Err(error) => Err(error),
    }
}

pub async fn update_provider_blog_post(
    conn: &mut AsyncPgConnection,
    provider_profile_id: Uuid,
    provider_blog_post_id: Uuid,
    update: ProviderBlogPostUpdate,
) -> Result<ProviderBlogPost, diesel::result::Error> {
    match diesel::update(
        provider_blog_posts::table
            .filter(provider_blog_posts::provider_profile_id.eq(provider_profile_id))
            .filter(provider_blog_posts::provider_blog_post_id.eq(provider_blog_post_id)),
    )
    .set((
        provider_blog_posts::provider_blog_post_slug.eq(update.provider_blog_post_slug),
        provider_blog_posts::provider_blog_post_title.eq(update.provider_blog_post_title),
        provider_blog_posts::provider_blog_post_excerpt.eq(update.provider_blog_post_excerpt),
        provider_blog_posts::provider_blog_post_body.eq(update.provider_blog_post_body),
        provider_blog_posts::provider_blog_post_status.eq(update.provider_blog_post_status),
        provider_blog_posts::provider_blog_post_moderation_status
            .eq(update.provider_blog_post_moderation_status),
        provider_blog_posts::provider_blog_post_published_at
            .eq(update.provider_blog_post_published_at),
        provider_blog_posts::provider_blog_post_updated_at.eq(update.provider_blog_post_updated_at),
    ))
    .returning(ProviderBlogPost::as_returning())
    .get_result::<ProviderBlogPost>(conn)
    .await
    {
        Ok(post) => Ok(post),
        Err(error) => Err(error),
    }
}

pub async fn update_provider_blog_post_moderation(
    conn: &mut AsyncPgConnection,
    provider_blog_post_id: Uuid,
    moderation_status: ModerationStatus,
    now: DateTime<Utc>,
) -> Result<ProviderBlogPost, diesel::result::Error> {
    match diesel::update(
        provider_blog_posts::table
            .filter(provider_blog_posts::provider_blog_post_id.eq(provider_blog_post_id)),
    )
    .set((
        provider_blog_posts::provider_blog_post_moderation_status.eq(moderation_status),
        provider_blog_posts::provider_blog_post_updated_at.eq(now),
    ))
    .returning(ProviderBlogPost::as_returning())
    .get_result::<ProviderBlogPost>(conn)
    .await
    {
        Ok(post) => Ok(post),
        Err(error) => Err(error),
    }
}
