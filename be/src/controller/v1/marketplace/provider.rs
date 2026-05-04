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
                CompleteImageUploadRequest, CreateImageRequest, CreateProviderBlogPostRequest,
                UpsertProviderProfileRequest,
            },
            response::{
                ImageResponse, ProviderBlogPostResponse, ProviderDetailResponse,
                ProviderProfileResponse,
            },
        },
    },
    init::state::server_state::ServerState,
    middleware::auth::AuthContext,
    service::marketplace::{provider, provider_media},
};

#[utoipa::path(
    get,
    path = "/api/v1/marketplace/provider/profile",
    tag = "marketplace-provider",
    responses((status = 200, description = "Provider dashboard profile", body = ApiEnvelope<ProviderDetailResponse>))
)]
pub async fn provider_get_profile(
    State(state): State<Arc<ServerState>>,
    Extension(auth_context): Extension<AuthContext>,
) -> ApiResponseResult<ProviderDetailResponse> {
    match provider::provider_profile(state, auth_context.claims).await {
        Ok(response) => Ok(api_ok(response)),
        Err(error) => Err(error),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/marketplace/provider/profile",
    tag = "marketplace-provider",
    request_body = UpsertProviderProfileRequest,
    responses((status = 200, description = "Updated provider profile", body = ApiEnvelope<ProviderProfileResponse>))
)]
pub async fn provider_upsert_profile(
    State(state): State<Arc<ServerState>>,
    Extension(auth_context): Extension<AuthContext>,
    Json(request): Json<UpsertProviderProfileRequest>,
) -> ApiResponseResult<ProviderProfileResponse> {
    match provider::upsert_provider_profile(state, auth_context.claims, request).await {
        Ok(response) => Ok(api_ok(response)),
        Err(error) => Err(error),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/marketplace/provider/blog",
    tag = "marketplace-provider",
    request_body = CreateProviderBlogPostRequest,
    responses((status = 201, description = "Created provider blog post", body = ApiEnvelope<ProviderBlogPostResponse>))
)]
pub async fn provider_create_blog_post(
    State(state): State<Arc<ServerState>>,
    Extension(auth_context): Extension<AuthContext>,
    Json(request): Json<CreateProviderBlogPostRequest>,
) -> ApiResponseResult<ProviderBlogPostResponse> {
    match provider::create_provider_blog_post(state, auth_context.claims, request).await {
        Ok(response) => Ok(api_created(response)),
        Err(error) => Err(error),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/marketplace/provider/images",
    tag = "marketplace-provider",
    request_body = CreateImageRequest,
    responses((status = 201, description = "Created provider image metadata", body = ApiEnvelope<ImageResponse>))
)]
pub async fn provider_create_image(
    State(state): State<Arc<ServerState>>,
    Extension(auth_context): Extension<AuthContext>,
    Json(request): Json<CreateImageRequest>,
) -> ApiResponseResult<ImageResponse> {
    match provider_media::create_provider_image(state, auth_context.claims, request).await {
        Ok(response) => Ok(api_created(response)),
        Err(error) => Err(error),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/marketplace/provider/images/{image_id}/complete",
    tag = "marketplace-provider",
    params(("image_id" = Uuid, Path, description = "Image id")),
    request_body = CompleteImageUploadRequest,
    responses((status = 200, description = "Completed image upload", body = ApiEnvelope<ImageResponse>))
)]
pub async fn provider_complete_image_upload(
    State(state): State<Arc<ServerState>>,
    Extension(auth_context): Extension<AuthContext>,
    Path(image_id): Path<Uuid>,
    Json(request): Json<CompleteImageUploadRequest>,
) -> ApiResponseResult<ImageResponse> {
    match provider_media::complete_provider_image_upload(
        state,
        auth_context.claims,
        image_id,
        request,
    )
    .await
    {
        Ok(response) => Ok(api_ok(response)),
        Err(error) => Err(error),
    }
}
