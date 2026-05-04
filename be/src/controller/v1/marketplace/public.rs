use std::sync::Arc;

use axum::extract::{Path, Query, State};

use crate::{
    domain::marketplace::enums::BannerPlacement,
    dto::{
        api_response::{ApiEnvelope, ApiResponseResult, api_ok},
        marketplace::{
            request::ProviderDirectoryQuery,
            response::{
                BannerListResponse, ProviderBlogPostResponse, ProviderDetailResponse,
                ProviderDirectoryResponse,
            },
        },
    },
    init::state::server_state::ServerState,
    service::marketplace::public,
};

#[utoipa::path(
    get,
    path = "/api/v1/marketplace/providers",
    tag = "marketplace",
    params(ProviderDirectoryQuery),
    responses((status = 200, description = "Published provider directory", body = ApiEnvelope<ProviderDirectoryResponse>))
)]
pub async fn public_provider_directory(
    State(state): State<Arc<ServerState>>,
    Query(query): Query<ProviderDirectoryQuery>,
) -> ApiResponseResult<ProviderDirectoryResponse> {
    match public::provider_directory(state, query).await {
        Ok(response) => Ok(api_ok(response)),
        Err(error) => Err(error),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/marketplace/providers/{slug}",
    tag = "marketplace",
    params(("slug" = String, Path, description = "Provider slug")),
    responses((status = 200, description = "Published provider profile", body = ApiEnvelope<ProviderDetailResponse>))
)]
pub async fn public_provider_detail(
    State(state): State<Arc<ServerState>>,
    Path(slug): Path<String>,
) -> ApiResponseResult<ProviderDetailResponse> {
    match public::provider_detail(state, slug).await {
        Ok(response) => Ok(api_ok(response)),
        Err(error) => Err(error),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/marketplace/providers/{provider_slug}/blog/{post_slug}",
    tag = "marketplace",
    params(
        ("provider_slug" = String, Path, description = "Provider slug"),
        ("post_slug" = String, Path, description = "Blog post slug")
    ),
    responses((status = 200, description = "Published provider blog post", body = ApiEnvelope<ProviderBlogPostResponse>))
)]
pub async fn public_provider_blog_post(
    State(state): State<Arc<ServerState>>,
    Path((provider_slug, post_slug)): Path<(String, String)>,
) -> ApiResponseResult<ProviderBlogPostResponse> {
    match public::provider_blog_post(state, provider_slug, post_slug).await {
        Ok(response) => Ok(api_ok(response)),
        Err(error) => Err(error),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/marketplace/banners/{placement}",
    tag = "marketplace",
    params(("placement" = BannerPlacement, Path, description = "Banner placement")),
    responses((status = 200, description = "Active banners", body = ApiEnvelope<BannerListResponse>))
)]
pub async fn marketplace_active_banners(
    State(state): State<Arc<ServerState>>,
    Path(placement): Path<BannerPlacement>,
) -> ApiResponseResult<BannerListResponse> {
    match public::active_banners(state, placement).await {
        Ok(response) => Ok(api_ok(response)),
        Err(error) => Err(error),
    }
}
