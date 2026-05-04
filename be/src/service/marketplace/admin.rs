use std::sync::Arc;

use chrono::Utc;
use uuid::Uuid;

use crate::{
    domain::{
        auth::jwt::AccessTokenClaims,
        marketplace::{
            enums::{BlogPostStatus, ModerationStatus},
            moderation::{NewAdvertisementBanner, NewCentralBlogPost, NewModerationBan},
        },
    },
    dto::{
        api_response::ApiResult,
        marketplace::{
            request::{CreateBanRequest, CreateBannerRequest, CreateCentralBlogPostRequest},
            response::{
                AdminOverviewResponse, BanListResponse, BanResponse, BannerResponse,
                CentralBlogPostResponse,
            },
        },
    },
    error::{api_error::ApiError, code_error::CodeError},
    init::state::server_state::ServerState,
    repository::marketplace::postgres::{admin_repository, payment_repository},
    service::{
        auth::datasource::postgres_conn,
        marketplace::{authz, validation},
    },
};

pub async fn create_ban(
    state: Arc<ServerState>,
    claims: AccessTokenClaims,
    request: CreateBanRequest,
) -> ApiResult<BanResponse> {
    match authz::require_moderator(&claims) {
        Ok(()) => {}
        Err(error) => return Err(error),
    }
    let reason = match validation::required_short(request.reason, "reason") {
        Ok(value) => value,
        Err(error) => return Err(error),
    };
    let starts_at = match request.starts_at {
        Some(starts_at) => starts_at,
        None => Utc::now(),
    };
    match request.expires_at {
        Some(expires_at) if expires_at <= starts_at => {
            return Err(ApiError::public(
                CodeError::VALIDATION_FAILED,
                "expires_at must be later than starts_at",
            ));
        }
        Some(_) | None => {}
    }

    let mut conn = match postgres_conn(&state).await {
        Ok(conn) => conn,
        Err(error) => return Err(error),
    };
    let new_ban = NewModerationBan {
        target_user_id: request.target_user_id,
        actor_user_id: claims.user_id,
        moderation_ban_scope: request.scope,
        moderation_ban_reason: reason,
        moderation_ban_starts_at: starts_at,
        moderation_ban_expires_at: request.expires_at,
    };

    match admin_repository::insert_ban(&mut conn, new_ban).await {
        Ok(ban) => Ok(BanResponse::from(ban)),
        Err(error) => Err(ApiError::from_source(CodeError::DB_INSERT_ERROR, error)),
    }
}

pub async fn revoke_ban(
    state: Arc<ServerState>,
    claims: AccessTokenClaims,
    ban_id: Uuid,
) -> ApiResult<BanResponse> {
    match authz::require_moderator(&claims) {
        Ok(()) => {}
        Err(error) => return Err(error),
    }
    let mut conn = match postgres_conn(&state).await {
        Ok(conn) => conn,
        Err(error) => return Err(error),
    };
    match admin_repository::revoke_ban(&mut conn, ban_id, claims.user_id, Utc::now()).await {
        Ok(ban) => Ok(BanResponse::from(ban)),
        Err(error) => Err(ApiError::from_source(CodeError::DB_UPDATE_ERROR, error)),
    }
}

pub async fn active_bans(
    state: Arc<ServerState>,
    claims: AccessTokenClaims,
) -> ApiResult<BanListResponse> {
    match authz::require_moderator(&claims) {
        Ok(()) => {}
        Err(error) => return Err(error),
    }
    let mut conn = match postgres_conn(&state).await {
        Ok(conn) => conn,
        Err(error) => return Err(error),
    };
    match admin_repository::list_active_bans(&mut conn, Utc::now()).await {
        Ok(bans) => Ok(BanListResponse {
            bans: bans.into_iter().map(Into::into).collect(),
        }),
        Err(error) => Err(ApiError::from_source(CodeError::DB_QUERY_ERROR, error)),
    }
}

pub async fn create_central_blog_post(
    state: Arc<ServerState>,
    claims: AccessTokenClaims,
    request: CreateCentralBlogPostRequest,
) -> ApiResult<CentralBlogPostResponse> {
    match authz::require_moderator(&claims) {
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
    let new_post = NewCentralBlogPost {
        author_user_id: claims.user_id,
        central_blog_post_slug: slug,
        central_blog_post_title: title,
        central_blog_post_excerpt: excerpt,
        central_blog_post_body: body,
        central_blog_post_status: request.status,
        central_blog_post_moderation_status: ModerationStatus::Approved,
        central_blog_post_published_at: published_at,
    };

    let mut conn = match postgres_conn(&state).await {
        Ok(conn) => conn,
        Err(error) => return Err(error),
    };
    match admin_repository::insert_central_blog_post(&mut conn, new_post).await {
        Ok(post) => Ok(CentralBlogPostResponse::from(post)),
        Err(error) => Err(ApiError::from_source(CodeError::DB_INSERT_ERROR, error)),
    }
}

pub async fn create_banner(
    state: Arc<ServerState>,
    claims: AccessTokenClaims,
    request: CreateBannerRequest,
) -> ApiResult<BannerResponse> {
    match authz::require_moderator(&claims) {
        Ok(()) => {}
        Err(error) => return Err(error),
    }
    let title = match validation::required_short(request.title, "title") {
        Ok(value) => value,
        Err(error) => return Err(error),
    };
    let target_url = match validation::required_short(request.target_url, "target_url") {
        Ok(value) => value,
        Err(error) => return Err(error),
    };
    match request.ends_at {
        Some(ends_at) if ends_at <= request.starts_at => {
            return Err(ApiError::public(
                CodeError::VALIDATION_FAILED,
                "ends_at must be later than starts_at",
            ));
        }
        Some(_) | None => {}
    }
    let new_banner = NewAdvertisementBanner {
        created_by_user_id: claims.user_id,
        advertisement_banner_placement: request.placement,
        advertisement_banner_status: request.status,
        advertisement_banner_title: title,
        advertisement_banner_target_url: target_url,
        advertisement_banner_priority: request.priority,
        advertisement_banner_starts_at: request.starts_at,
        advertisement_banner_ends_at: request.ends_at,
    };

    let mut conn = match postgres_conn(&state).await {
        Ok(conn) => conn,
        Err(error) => return Err(error),
    };
    match admin_repository::insert_banner(&mut conn, new_banner).await {
        Ok(banner) => Ok(BannerResponse::from(banner)),
        Err(error) => Err(ApiError::from_source(CodeError::DB_INSERT_ERROR, error)),
    }
}

pub async fn overview(
    state: Arc<ServerState>,
    claims: AccessTokenClaims,
) -> ApiResult<AdminOverviewResponse> {
    match authz::require_moderator(&claims) {
        Ok(()) => {}
        Err(error) => return Err(error),
    }
    let mut conn = match postgres_conn(&state).await {
        Ok(conn) => conn,
        Err(error) => return Err(error),
    };
    let now = Utc::now();
    let provider_count = match admin_repository::count_provider_profiles(&mut conn).await {
        Ok(count) => count,
        Err(error) => return Err(ApiError::from_source(CodeError::DB_QUERY_ERROR, error)),
    };
    let active_ban_count = match admin_repository::count_active_bans(&mut conn, now).await {
        Ok(count) => count,
        Err(error) => return Err(ApiError::from_source(CodeError::DB_QUERY_ERROR, error)),
    };
    let payment_intent_count = match payment_repository::count_payment_intents(&mut conn).await {
        Ok(count) => count,
        Err(error) => return Err(ApiError::from_source(CodeError::DB_QUERY_ERROR, error)),
    };
    let active_banner_count = match admin_repository::count_active_banners(&mut conn, now).await {
        Ok(count) => count,
        Err(error) => return Err(ApiError::from_source(CodeError::DB_QUERY_ERROR, error)),
    };

    Ok(AdminOverviewResponse {
        provider_count,
        active_ban_count,
        payment_intent_count,
        active_banner_count,
    })
}
