use std::sync::Arc;

use crate::{
    domain::{
        auth::jwt::AccessTokenClaims,
        marketplace::{
            enums::{BlogPostStatus, ModerationStatus},
            provider::{
                NewProviderBlogPost, NewProviderProfile, ProviderBlogPostUpdate,
                ProviderProfileUpdate,
            },
        },
    },
    dto::{
        api_response::ApiResult,
        marketplace::{
            request::{
                CreateProviderBlogPostRequest, UpdateProviderBlogPostRequest,
                UpsertProviderProfileRequest,
            },
            response::{ProviderBlogPostResponse, ProviderDetailResponse, ProviderProfileResponse},
        },
    },
    error::{api_error::ApiError, code_error::CodeError},
    init::state::server_state::ServerState,
    repository::marketplace::postgres::{
        media_repository, moderation_repository, provider_repository,
    },
    service::{
        auth::datasource::postgres_conn,
        marketplace::{authz, cache, indexing, validation},
    },
};
use chrono::Utc;

pub async fn provider_profile(
    state: Arc<ServerState>,
    claims: AccessTokenClaims,
) -> ApiResult<ProviderDetailResponse> {
    match authz::require_provider(&claims) {
        Ok(()) => {}
        Err(error) => return Err(error),
    }
    let mut conn = match postgres_conn(&state).await {
        Ok(conn) => conn,
        Err(error) => return Err(error),
    };

    let profile =
        match provider_repository::find_provider_profile_by_user(&mut conn, claims.user_id).await {
            Ok(Some(profile)) => profile,
            Ok(None) => return Err(ApiError::new(CodeError::MARKETPLACE_NOT_FOUND)),
            Err(error) => return Err(ApiError::from_source(CodeError::DB_QUERY_ERROR, error)),
        };

    let images = match media_repository::list_provider_profile_images(
        &mut conn,
        profile.provider_profile_id,
    )
    .await
    {
        Ok(images) => images.into_iter().map(Into::into).collect(),
        Err(error) => return Err(ApiError::from_source(CodeError::DB_QUERY_ERROR, error)),
    };

    let blog_posts =
        match provider_repository::list_provider_blog_posts(&mut conn, profile.provider_profile_id)
            .await
        {
            Ok(posts) => posts.into_iter().map(Into::into).collect(),
            Err(error) => return Err(ApiError::from_source(CodeError::DB_QUERY_ERROR, error)),
        };

    Ok(ProviderDetailResponse {
        profile: ProviderProfileResponse::from(profile),
        images,
        blog_posts,
    })
}

pub async fn upsert_provider_profile(
    state: Arc<ServerState>,
    claims: AccessTokenClaims,
    request: UpsertProviderProfileRequest,
) -> ApiResult<ProviderProfileResponse> {
    match authz::require_provider(&claims) {
        Ok(()) => {}
        Err(error) => return Err(error),
    }
    let display_name = match validation::required_short(request.display_name, "display_name") {
        Ok(value) => value,
        Err(error) => return Err(error),
    };
    let slug = match validation::slug_from(request.slug, &display_name) {
        Ok(value) => value,
        Err(error) => return Err(error),
    };
    let headline = match validation::short_optional(request.headline, "headline") {
        Ok(value) => value,
        Err(error) => return Err(error),
    };
    let bio = match validation::long_optional(request.bio, "bio") {
        Ok(value) => value,
        Err(error) => return Err(error),
    };
    let service_area = match validation::short_optional(request.service_area, "service_area") {
        Ok(value) => value,
        Err(error) => return Err(error),
    };

    let mut conn = match postgres_conn(&state).await {
        Ok(conn) => conn,
        Err(error) => return Err(error),
    };

    let existing =
        match provider_repository::find_provider_profile_by_user(&mut conn, claims.user_id).await {
            Ok(existing) => existing,
            Err(error) => return Err(ApiError::from_source(CodeError::DB_QUERY_ERROR, error)),
        };

    let profile = match existing {
        Some(_) => {
            match provider_repository::update_provider_profile_by_user(
                &mut conn,
                claims.user_id,
                ProviderProfileUpdate {
                    provider_profile_slug: slug,
                    provider_profile_display_name: display_name,
                    provider_profile_headline: headline,
                    provider_profile_bio: bio,
                    provider_profile_service_area: service_area,
                    provider_profile_status: request.status,
                    provider_profile_updated_at: Utc::now(),
                },
            )
            .await
            {
                Ok(profile) => profile,
                Err(error) => return Err(ApiError::from_source(CodeError::DB_UPDATE_ERROR, error)),
            }
        }
        None => {
            let new_profile = NewProviderProfile {
                user_id: claims.user_id,
                provider_profile_slug: slug,
                provider_profile_display_name: display_name,
                provider_profile_headline: headline,
                provider_profile_bio: bio,
                provider_profile_service_area: service_area,
                provider_profile_status: request.status,
                provider_profile_moderation_status: ModerationStatus::Pending,
            };
            match provider_repository::insert_provider_profile(&mut conn, new_profile).await {
                Ok(profile) => profile,
                Err(error) => return Err(ApiError::from_source(CodeError::DB_INSERT_ERROR, error)),
            }
        }
    };

    cache::clear_public_cache(&state, "provider_profile_upsert").await;
    indexing::rebuild_search_index(&state, "provider_profile_upsert").await;
    Ok(ProviderProfileResponse::from(profile))
}

pub async fn create_provider_blog_post(
    state: Arc<ServerState>,
    claims: AccessTokenClaims,
    request: CreateProviderBlogPostRequest,
) -> ApiResult<ProviderBlogPostResponse> {
    match authz::require_provider(&claims) {
        Ok(()) => {}
        Err(error) => return Err(error),
    }
    let title = match validation::required_short(request.title, "title") {
        Ok(value) => value,
        Err(error) => return Err(error),
    };
    let body = match validation::required_long(request.body, "body") {
        Ok(value) => value,
        Err(error) => return Err(error),
    };
    let excerpt = match validation::short_optional(request.excerpt, "excerpt") {
        Ok(value) => value,
        Err(error) => return Err(error),
    };
    let slug = match validation::slug_from(request.slug, &title) {
        Ok(value) => value,
        Err(error) => return Err(error),
    };

    let mut conn = match postgres_conn(&state).await {
        Ok(conn) => conn,
        Err(error) => return Err(error),
    };
    let profile =
        match provider_repository::find_provider_profile_by_user(&mut conn, claims.user_id).await {
            Ok(Some(profile)) => profile,
            Ok(None) => return Err(ApiError::new(CodeError::MARKETPLACE_NOT_FOUND)),
            Err(error) => return Err(ApiError::from_source(CodeError::DB_QUERY_ERROR, error)),
        };
    let published_at = match request.status {
        BlogPostStatus::Published => Some(Utc::now()),
        BlogPostStatus::Draft | BlogPostStatus::Archived => None,
    };
    let new_post = NewProviderBlogPost {
        provider_profile_id: profile.provider_profile_id,
        provider_blog_post_slug: slug,
        provider_blog_post_title: title,
        provider_blog_post_excerpt: excerpt,
        provider_blog_post_body: body,
        provider_blog_post_status: request.status,
        provider_blog_post_moderation_status: ModerationStatus::Pending,
        provider_blog_post_published_at: published_at,
    };

    match provider_repository::insert_provider_blog_post(&mut conn, new_post).await {
        Ok(post) => {
            cache::clear_public_cache(&state, "provider_blog_post_create").await;
            indexing::rebuild_search_index(&state, "provider_blog_post_create").await;
            Ok(ProviderBlogPostResponse::from(post))
        }
        Err(error) => Err(ApiError::from_source(CodeError::DB_INSERT_ERROR, error)),
    }
}

pub async fn update_provider_blog_post(
    state: Arc<ServerState>,
    claims: AccessTokenClaims,
    provider_blog_post_id: uuid::Uuid,
    request: UpdateProviderBlogPostRequest,
) -> ApiResult<ProviderBlogPostResponse> {
    match authz::require_provider(&claims) {
        Ok(()) => {}
        Err(error) => return Err(error),
    }
    let title = match validation::required_short(request.title, "title") {
        Ok(value) => value,
        Err(error) => return Err(error),
    };
    let body = match validation::required_long(request.body, "body") {
        Ok(value) => value,
        Err(error) => return Err(error),
    };
    let excerpt = match validation::short_optional(request.excerpt, "excerpt") {
        Ok(value) => value,
        Err(error) => return Err(error),
    };
    let slug = match validation::slug_from(request.slug, &title) {
        Ok(value) => value,
        Err(error) => return Err(error),
    };
    let published_at = match request.status {
        BlogPostStatus::Published => Some(Utc::now()),
        BlogPostStatus::Draft | BlogPostStatus::Archived => None,
    };

    let mut conn = match postgres_conn(&state).await {
        Ok(conn) => conn,
        Err(error) => return Err(error),
    };
    let profile =
        match provider_repository::find_provider_profile_by_user(&mut conn, claims.user_id).await {
            Ok(Some(profile)) => profile,
            Ok(None) => return Err(ApiError::new(CodeError::MARKETPLACE_NOT_FOUND)),
            Err(error) => return Err(ApiError::from_source(CodeError::DB_QUERY_ERROR, error)),
        };

    match moderation_repository::update_provider_blog_post(
        &mut conn,
        profile.provider_profile_id,
        provider_blog_post_id,
        ProviderBlogPostUpdate {
            provider_blog_post_slug: slug,
            provider_blog_post_title: title,
            provider_blog_post_excerpt: excerpt,
            provider_blog_post_body: body,
            provider_blog_post_status: request.status,
            provider_blog_post_moderation_status: ModerationStatus::Pending,
            provider_blog_post_published_at: published_at,
            provider_blog_post_updated_at: Utc::now(),
        },
    )
    .await
    {
        Ok(post) => {
            cache::clear_public_cache(&state, "provider_blog_post_update").await;
            indexing::rebuild_search_index(&state, "provider_blog_post_update").await;
            Ok(ProviderBlogPostResponse::from(post))
        }
        Err(error) => Err(ApiError::from_source(CodeError::DB_UPDATE_ERROR, error)),
    }
}
