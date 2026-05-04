use chrono::{DateTime, Utc};
use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::{
    domain::marketplace::{
        enums::{BannerPlacement, BannerStatus, BlogPostStatus, ModerationStatus},
        moderation::{
            AdvertisementBanner, CentralBlogPost, ModerationBan, NewAdvertisementBanner,
            NewCentralBlogPost, NewModerationBan,
        },
    },
    schema::{advertisement_banners, central_blog_posts, moderation_bans, provider_profiles},
};

pub async fn insert_ban(
    conn: &mut AsyncPgConnection,
    new_ban: NewModerationBan,
) -> Result<ModerationBan, diesel::result::Error> {
    match diesel::insert_into(moderation_bans::table)
        .values(new_ban)
        .returning(ModerationBan::as_returning())
        .get_result::<ModerationBan>(conn)
        .await
    {
        Ok(ban) => Ok(ban),
        Err(error) => Err(error),
    }
}

pub async fn revoke_ban(
    conn: &mut AsyncPgConnection,
    ban_id: Uuid,
    revoked_by_user_id: Uuid,
    now: DateTime<Utc>,
) -> Result<ModerationBan, diesel::result::Error> {
    match diesel::update(
        moderation_bans::table.filter(moderation_bans::moderation_ban_id.eq(ban_id)),
    )
    .set((
        moderation_bans::moderation_ban_revoked_at.eq(now),
        moderation_bans::revoked_by_user_id.eq(revoked_by_user_id),
    ))
    .returning(ModerationBan::as_returning())
    .get_result::<ModerationBan>(conn)
    .await
    {
        Ok(ban) => Ok(ban),
        Err(error) => Err(error),
    }
}

pub async fn list_active_bans(
    conn: &mut AsyncPgConnection,
    now: DateTime<Utc>,
) -> Result<Vec<ModerationBan>, diesel::result::Error> {
    match moderation_bans::table
        .filter(moderation_bans::moderation_ban_revoked_at.is_null())
        .filter(moderation_bans::moderation_ban_starts_at.le(now))
        .filter(
            moderation_bans::moderation_ban_expires_at
                .is_null()
                .or(moderation_bans::moderation_ban_expires_at.gt(now)),
        )
        .order(moderation_bans::moderation_ban_created_at.desc())
        .select(ModerationBan::as_select())
        .load::<ModerationBan>(conn)
        .await
    {
        Ok(bans) => Ok(bans),
        Err(error) => Err(error),
    }
}

pub async fn insert_central_blog_post(
    conn: &mut AsyncPgConnection,
    new_post: NewCentralBlogPost,
) -> Result<CentralBlogPost, diesel::result::Error> {
    match diesel::insert_into(central_blog_posts::table)
        .values(new_post)
        .returning(CentralBlogPost::as_returning())
        .get_result::<CentralBlogPost>(conn)
        .await
    {
        Ok(post) => Ok(post),
        Err(error) => Err(error),
    }
}

pub async fn list_public_central_blog_posts_for_search(
    conn: &mut AsyncPgConnection,
) -> Result<Vec<CentralBlogPost>, diesel::result::Error> {
    match central_blog_posts::table
        .filter(central_blog_posts::central_blog_post_status.eq(BlogPostStatus::Published))
        .filter(
            central_blog_posts::central_blog_post_moderation_status.eq(ModerationStatus::Approved),
        )
        .order(central_blog_posts::central_blog_post_published_at.desc())
        .select(CentralBlogPost::as_select())
        .load::<CentralBlogPost>(conn)
        .await
    {
        Ok(posts) => Ok(posts),
        Err(error) => Err(error),
    }
}

pub async fn insert_banner(
    conn: &mut AsyncPgConnection,
    new_banner: NewAdvertisementBanner,
) -> Result<AdvertisementBanner, diesel::result::Error> {
    match diesel::insert_into(advertisement_banners::table)
        .values(new_banner)
        .returning(AdvertisementBanner::as_returning())
        .get_result::<AdvertisementBanner>(conn)
        .await
    {
        Ok(banner) => Ok(banner),
        Err(error) => Err(error),
    }
}

pub async fn list_active_banners(
    conn: &mut AsyncPgConnection,
    placement: BannerPlacement,
    now: DateTime<Utc>,
) -> Result<Vec<AdvertisementBanner>, diesel::result::Error> {
    match advertisement_banners::table
        .filter(advertisement_banners::advertisement_banner_placement.eq(placement))
        .filter(advertisement_banners::advertisement_banner_status.eq(BannerStatus::Active))
        .filter(advertisement_banners::advertisement_banner_starts_at.le(now))
        .filter(
            advertisement_banners::advertisement_banner_ends_at
                .is_null()
                .or(advertisement_banners::advertisement_banner_ends_at.gt(now)),
        )
        .order(advertisement_banners::advertisement_banner_priority.desc())
        .select(AdvertisementBanner::as_select())
        .load::<AdvertisementBanner>(conn)
        .await
    {
        Ok(banners) => Ok(banners),
        Err(error) => Err(error),
    }
}

pub async fn count_provider_profiles(
    conn: &mut AsyncPgConnection,
) -> Result<i64, diesel::result::Error> {
    match provider_profiles::table
        .count()
        .get_result::<i64>(conn)
        .await
    {
        Ok(count) => Ok(count),
        Err(error) => Err(error),
    }
}

pub async fn count_active_bans(
    conn: &mut AsyncPgConnection,
    now: DateTime<Utc>,
) -> Result<i64, diesel::result::Error> {
    match moderation_bans::table
        .filter(moderation_bans::moderation_ban_revoked_at.is_null())
        .filter(moderation_bans::moderation_ban_starts_at.le(now))
        .filter(
            moderation_bans::moderation_ban_expires_at
                .is_null()
                .or(moderation_bans::moderation_ban_expires_at.gt(now)),
        )
        .count()
        .get_result::<i64>(conn)
        .await
    {
        Ok(count) => Ok(count),
        Err(error) => Err(error),
    }
}

pub async fn count_active_banners(
    conn: &mut AsyncPgConnection,
    now: DateTime<Utc>,
) -> Result<i64, diesel::result::Error> {
    match advertisement_banners::table
        .filter(advertisement_banners::advertisement_banner_status.eq(BannerStatus::Active))
        .filter(advertisement_banners::advertisement_banner_starts_at.le(now))
        .filter(
            advertisement_banners::advertisement_banner_ends_at
                .is_null()
                .or(advertisement_banners::advertisement_banner_ends_at.gt(now)),
        )
        .count()
        .get_result::<i64>(conn)
        .await
    {
        Ok(count) => Ok(count),
        Err(error) => Err(error),
    }
}
