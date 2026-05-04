use std::sync::Arc;

use axum::{
    Extension, Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::{
    dto::{
        api_response::{ApiEnvelope, ApiResponseResult, api_created, api_ok},
        marketplace::{
            request::{
                CreateBanRequest, CreateBannerRequest, CreateCentralBlogPostRequest,
                ModerationDecisionRequest,
            },
            response::{
                AdminOverviewResponse, BanListResponse, BanResponse, BannerResponse,
                CentralBlogPostResponse, MarketplaceCacheClearResponse,
                MarketplaceSearchReindexResponse, ProviderBlogPostResponse,
                ProviderProfileResponse,
            },
        },
    },
    init::state::server_state::ServerState,
    middleware::auth::AuthContext,
    service::marketplace::{admin, moderation, search},
};

#[utoipa::path(
    get,
    path = "/api/v1/marketplace/admin/overview",
    tag = "marketplace-admin",
    responses((status = 200, description = "Marketplace admin overview", body = ApiEnvelope<AdminOverviewResponse>))
)]
pub async fn admin_marketplace_overview(
    State(state): State<Arc<ServerState>>,
    Extension(auth_context): Extension<AuthContext>,
) -> ApiResponseResult<AdminOverviewResponse> {
    match admin::overview(state, auth_context.claims).await {
        Ok(response) => Ok(api_ok(response)),
        Err(error) => Err(error),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/marketplace/admin/search/reindex",
    tag = "marketplace-admin",
    responses((status = 200, description = "Rebuilt marketplace search index", body = ApiEnvelope<MarketplaceSearchReindexResponse>))
)]
pub async fn admin_reindex_marketplace_search(
    State(state): State<Arc<ServerState>>,
    Extension(auth_context): Extension<AuthContext>,
) -> ApiResponseResult<MarketplaceSearchReindexResponse> {
    match search::reindex_marketplace_search(state, auth_context.claims).await {
        Ok(response) => Ok(api_ok(response)),
        Err(error) => Err(error),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/marketplace/admin/cache/clear",
    tag = "marketplace-admin",
    responses((status = 200, description = "Cleared marketplace public cache", body = ApiEnvelope<MarketplaceCacheClearResponse>))
)]
pub async fn admin_clear_marketplace_public_cache(
    State(state): State<Arc<ServerState>>,
    Extension(auth_context): Extension<AuthContext>,
) -> ApiResponseResult<MarketplaceCacheClearResponse> {
    match admin::clear_marketplace_public_cache(state, auth_context.claims).await {
        Ok(response) => Ok(api_ok(response)),
        Err(error) => Err(error),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/marketplace/admin/providers/{provider_profile_id}/moderation",
    tag = "marketplace-admin",
    params(("provider_profile_id" = Uuid, Path, description = "Provider profile id")),
    request_body = ModerationDecisionRequest,
    responses((status = 200, description = "Moderated provider profile", body = ApiEnvelope<ProviderProfileResponse>))
)]
pub async fn admin_moderate_provider_profile(
    State(state): State<Arc<ServerState>>,
    Extension(auth_context): Extension<AuthContext>,
    Path(provider_profile_id): Path<Uuid>,
    Json(request): Json<ModerationDecisionRequest>,
) -> ApiResponseResult<ProviderProfileResponse> {
    match moderation::moderate_provider_profile(
        state,
        auth_context.claims,
        provider_profile_id,
        request,
    )
    .await
    {
        Ok(response) => Ok(api_ok(response)),
        Err(error) => Err(error),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/marketplace/admin/provider-blog/{provider_blog_post_id}/moderation",
    tag = "marketplace-admin",
    params(("provider_blog_post_id" = Uuid, Path, description = "Provider blog post id")),
    request_body = ModerationDecisionRequest,
    responses((status = 200, description = "Moderated provider blog post", body = ApiEnvelope<ProviderBlogPostResponse>))
)]
pub async fn admin_moderate_provider_blog_post(
    State(state): State<Arc<ServerState>>,
    Extension(auth_context): Extension<AuthContext>,
    Path(provider_blog_post_id): Path<Uuid>,
    Json(request): Json<ModerationDecisionRequest>,
) -> ApiResponseResult<ProviderBlogPostResponse> {
    match moderation::moderate_provider_blog_post(
        state,
        auth_context.claims,
        provider_blog_post_id,
        request,
    )
    .await
    {
        Ok(response) => Ok(api_ok(response)),
        Err(error) => Err(error),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/marketplace/admin/bans/active",
    tag = "marketplace-admin",
    responses((status = 200, description = "Active moderation bans", body = ApiEnvelope<BanListResponse>))
)]
pub async fn admin_active_bans(
    State(state): State<Arc<ServerState>>,
    Extension(auth_context): Extension<AuthContext>,
) -> ApiResponseResult<BanListResponse> {
    match admin::active_bans(state, auth_context.claims).await {
        Ok(response) => Ok(api_ok(response)),
        Err(error) => Err(error),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/marketplace/admin/bans",
    tag = "marketplace-admin",
    request_body = CreateBanRequest,
    responses((status = 201, description = "Created moderation ban", body = ApiEnvelope<BanResponse>))
)]
pub async fn admin_create_ban(
    State(state): State<Arc<ServerState>>,
    Extension(auth_context): Extension<AuthContext>,
    Json(request): Json<CreateBanRequest>,
) -> ApiResponseResult<BanResponse> {
    match admin::create_ban(state, auth_context.claims, request).await {
        Ok(response) => Ok(api_created(response)),
        Err(error) => Err(error),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/marketplace/admin/bans/{ban_id}/revoke",
    tag = "marketplace-admin",
    params(("ban_id" = Uuid, Path, description = "Moderation ban id")),
    responses((status = 200, description = "Revoked moderation ban", body = ApiEnvelope<BanResponse>))
)]
pub async fn admin_revoke_ban(
    State(state): State<Arc<ServerState>>,
    Extension(auth_context): Extension<AuthContext>,
    Path(ban_id): Path<Uuid>,
) -> ApiResponseResult<BanResponse> {
    match admin::revoke_ban(state, auth_context.claims, ban_id).await {
        Ok(response) => Ok(api_ok(response)),
        Err(error) => Err(error),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/marketplace/admin/blog",
    tag = "marketplace-admin",
    request_body = CreateCentralBlogPostRequest,
    responses((status = 201, description = "Created central blog post", body = ApiEnvelope<CentralBlogPostResponse>))
)]
pub async fn admin_create_central_blog_post(
    State(state): State<Arc<ServerState>>,
    Extension(auth_context): Extension<AuthContext>,
    Json(request): Json<CreateCentralBlogPostRequest>,
) -> ApiResponseResult<CentralBlogPostResponse> {
    match admin::create_central_blog_post(state, auth_context.claims, request).await {
        Ok(response) => Ok(api_created(response)),
        Err(error) => Err(error),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/marketplace/admin/banners",
    tag = "marketplace-admin",
    request_body = CreateBannerRequest,
    responses((status = 201, description = "Created advertisement banner", body = ApiEnvelope<BannerResponse>))
)]
pub async fn admin_create_banner(
    State(state): State<Arc<ServerState>>,
    Extension(auth_context): Extension<AuthContext>,
    Json(request): Json<CreateBannerRequest>,
) -> ApiResponseResult<BannerResponse> {
    match admin::create_banner(state, auth_context.claims, request).await {
        Ok(response) => Ok(api_created(response)),
        Err(error) => Err(error),
    }
}
