use chrono::{DateTime, Utc};
use diesel::{
    ExpressionMethods, OptionalExtension, PgTextExpressionMethods, QueryDsl, SelectableHelper,
};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::{
    domain::marketplace::{
        enums::{BlogPostStatus, ModerationStatus, ProviderProfileStatus},
        provider::{
            NewProviderBlogPost, NewProviderProfile, NewUserProfileExtension, ProviderBlogPost,
            ProviderProfile, ProviderProfileUpdate, UserProfileExtension,
        },
    },
    schema::{provider_blog_posts, provider_profiles, user_profile_extensions},
};

pub async fn find_user_profile_extension(
    conn: &mut AsyncPgConnection,
    user_id: Uuid,
) -> Result<Option<UserProfileExtension>, diesel::result::Error> {
    match user_profile_extensions::table
        .filter(user_profile_extensions::user_id.eq(user_id))
        .select(UserProfileExtension::as_select())
        .first::<UserProfileExtension>(conn)
        .await
        .optional()
    {
        Ok(profile) => Ok(profile),
        Err(error) => Err(error),
    }
}

pub async fn insert_user_profile_extension(
    conn: &mut AsyncPgConnection,
    new_profile: NewUserProfileExtension,
) -> Result<UserProfileExtension, diesel::result::Error> {
    match diesel::insert_into(user_profile_extensions::table)
        .values(new_profile)
        .returning(UserProfileExtension::as_returning())
        .get_result::<UserProfileExtension>(conn)
        .await
    {
        Ok(profile) => Ok(profile),
        Err(error) => Err(error),
    }
}

pub async fn update_user_profile_extension(
    conn: &mut AsyncPgConnection,
    user_id: Uuid,
    display_name: Option<String>,
    bio: Option<String>,
    phone: Option<String>,
    public_email: Option<String>,
    now: DateTime<Utc>,
) -> Result<UserProfileExtension, diesel::result::Error> {
    match diesel::update(
        user_profile_extensions::table.filter(user_profile_extensions::user_id.eq(user_id)),
    )
    .set((
        user_profile_extensions::user_profile_extension_display_name.eq(display_name),
        user_profile_extensions::user_profile_extension_bio.eq(bio),
        user_profile_extensions::user_profile_extension_phone.eq(phone),
        user_profile_extensions::user_profile_extension_public_email.eq(public_email),
        user_profile_extensions::user_profile_extension_updated_at.eq(now),
    ))
    .returning(UserProfileExtension::as_returning())
    .get_result::<UserProfileExtension>(conn)
    .await
    {
        Ok(profile) => Ok(profile),
        Err(error) => Err(error),
    }
}

pub async fn find_provider_profile_by_user(
    conn: &mut AsyncPgConnection,
    user_id: Uuid,
) -> Result<Option<ProviderProfile>, diesel::result::Error> {
    match provider_profiles::table
        .filter(provider_profiles::user_id.eq(user_id))
        .select(ProviderProfile::as_select())
        .first::<ProviderProfile>(conn)
        .await
        .optional()
    {
        Ok(profile) => Ok(profile),
        Err(error) => Err(error),
    }
}

pub async fn find_provider_profile_by_id(
    conn: &mut AsyncPgConnection,
    provider_profile_id: Uuid,
) -> Result<Option<ProviderProfile>, diesel::result::Error> {
    match provider_profiles::table
        .filter(provider_profiles::provider_profile_id.eq(provider_profile_id))
        .select(ProviderProfile::as_select())
        .first::<ProviderProfile>(conn)
        .await
        .optional()
    {
        Ok(profile) => Ok(profile),
        Err(error) => Err(error),
    }
}

pub async fn find_public_provider_by_slug(
    conn: &mut AsyncPgConnection,
    slug: &str,
) -> Result<Option<ProviderProfile>, diesel::result::Error> {
    match provider_profiles::table
        .filter(provider_profiles::provider_profile_slug.eq(slug))
        .filter(provider_profiles::provider_profile_status.eq(ProviderProfileStatus::Published))
        .filter(
            provider_profiles::provider_profile_moderation_status.eq(ModerationStatus::Approved),
        )
        .select(ProviderProfile::as_select())
        .first::<ProviderProfile>(conn)
        .await
        .optional()
    {
        Ok(profile) => Ok(profile),
        Err(error) => Err(error),
    }
}

pub async fn list_public_providers(
    conn: &mut AsyncPgConnection,
    q: Option<String>,
    service_area: Option<String>,
    limit: i64,
) -> Result<Vec<ProviderProfile>, diesel::result::Error> {
    let mut query = provider_profiles::table
        .filter(provider_profiles::provider_profile_status.eq(ProviderProfileStatus::Published))
        .filter(
            provider_profiles::provider_profile_moderation_status.eq(ModerationStatus::Approved),
        )
        .into_boxed();

    if let Some(value) = q {
        let trimmed = value.trim();
        if !trimmed.is_empty() {
            let pattern = format!("%{trimmed}%");
            query = query.filter(provider_profiles::provider_profile_display_name.ilike(pattern));
        }
    }

    if let Some(value) = service_area {
        let trimmed = value.trim();
        if !trimmed.is_empty() {
            let pattern = format!("%{trimmed}%");
            query = query.filter(provider_profiles::provider_profile_service_area.ilike(pattern));
        }
    }

    match query
        .order(provider_profiles::provider_profile_updated_at.desc())
        .limit(limit)
        .select(ProviderProfile::as_select())
        .load::<ProviderProfile>(conn)
        .await
    {
        Ok(profiles) => Ok(profiles),
        Err(error) => Err(error),
    }
}

pub async fn insert_provider_profile(
    conn: &mut AsyncPgConnection,
    new_profile: NewProviderProfile,
) -> Result<ProviderProfile, diesel::result::Error> {
    match diesel::insert_into(provider_profiles::table)
        .values(new_profile)
        .returning(ProviderProfile::as_returning())
        .get_result::<ProviderProfile>(conn)
        .await
    {
        Ok(profile) => Ok(profile),
        Err(error) => Err(error),
    }
}

pub async fn update_provider_profile_by_user(
    conn: &mut AsyncPgConnection,
    user_id: Uuid,
    update: ProviderProfileUpdate,
) -> Result<ProviderProfile, diesel::result::Error> {
    match diesel::update(provider_profiles::table.filter(provider_profiles::user_id.eq(user_id)))
        .set((
            provider_profiles::provider_profile_slug.eq(update.provider_profile_slug),
            provider_profiles::provider_profile_display_name
                .eq(update.provider_profile_display_name),
            provider_profiles::provider_profile_headline.eq(update.provider_profile_headline),
            provider_profiles::provider_profile_bio.eq(update.provider_profile_bio),
            provider_profiles::provider_profile_service_area
                .eq(update.provider_profile_service_area),
            provider_profiles::provider_profile_status.eq(update.provider_profile_status),
            provider_profiles::provider_profile_updated_at.eq(update.provider_profile_updated_at),
        ))
        .returning(ProviderProfile::as_returning())
        .get_result::<ProviderProfile>(conn)
        .await
    {
        Ok(profile) => Ok(profile),
        Err(error) => Err(error),
    }
}

pub async fn insert_provider_blog_post(
    conn: &mut AsyncPgConnection,
    new_post: NewProviderBlogPost,
) -> Result<ProviderBlogPost, diesel::result::Error> {
    match diesel::insert_into(provider_blog_posts::table)
        .values(new_post)
        .returning(ProviderBlogPost::as_returning())
        .get_result::<ProviderBlogPost>(conn)
        .await
    {
        Ok(post) => Ok(post),
        Err(error) => Err(error),
    }
}

pub async fn list_provider_blog_posts(
    conn: &mut AsyncPgConnection,
    provider_profile_id: Uuid,
) -> Result<Vec<ProviderBlogPost>, diesel::result::Error> {
    match provider_blog_posts::table
        .filter(provider_blog_posts::provider_profile_id.eq(provider_profile_id))
        .order(provider_blog_posts::provider_blog_post_updated_at.desc())
        .select(ProviderBlogPost::as_select())
        .load::<ProviderBlogPost>(conn)
        .await
    {
        Ok(posts) => Ok(posts),
        Err(error) => Err(error),
    }
}

pub async fn list_public_provider_blog_posts(
    conn: &mut AsyncPgConnection,
    provider_profile_id: Uuid,
) -> Result<Vec<ProviderBlogPost>, diesel::result::Error> {
    match provider_blog_posts::table
        .filter(provider_blog_posts::provider_profile_id.eq(provider_profile_id))
        .filter(provider_blog_posts::provider_blog_post_status.eq(BlogPostStatus::Published))
        .filter(
            provider_blog_posts::provider_blog_post_moderation_status
                .eq(ModerationStatus::Approved),
        )
        .order(provider_blog_posts::provider_blog_post_published_at.desc())
        .select(ProviderBlogPost::as_select())
        .load::<ProviderBlogPost>(conn)
        .await
    {
        Ok(posts) => Ok(posts),
        Err(error) => Err(error),
    }
}

pub async fn find_public_provider_blog_post(
    conn: &mut AsyncPgConnection,
    provider_profile_id: Uuid,
    slug: &str,
) -> Result<Option<ProviderBlogPost>, diesel::result::Error> {
    match provider_blog_posts::table
        .filter(provider_blog_posts::provider_profile_id.eq(provider_profile_id))
        .filter(provider_blog_posts::provider_blog_post_slug.eq(slug))
        .filter(provider_blog_posts::provider_blog_post_status.eq(BlogPostStatus::Published))
        .filter(
            provider_blog_posts::provider_blog_post_moderation_status
                .eq(ModerationStatus::Approved),
        )
        .select(ProviderBlogPost::as_select())
        .first::<ProviderBlogPost>(conn)
        .await
        .optional()
    {
        Ok(post) => Ok(post),
        Err(error) => Err(error),
    }
}
