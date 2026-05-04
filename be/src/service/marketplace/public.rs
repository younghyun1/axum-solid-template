use std::sync::Arc;

use chrono::Utc;

use crate::{
    dto::{
        api_response::ApiResult,
        marketplace::{
            request::ProviderDirectoryQuery,
            response::{
                BannerListResponse, ProviderBlogPostResponse, ProviderDetailResponse,
                ProviderDirectoryCardResponse, ProviderDirectoryResponse, ProviderProfileResponse,
            },
        },
    },
    error::{api_error::ApiError, code_error::CodeError},
    init::state::server_state::ServerState,
    repository::marketplace::postgres::{admin_repository, media_repository, provider_repository},
    service::{auth::datasource::postgres_conn, marketplace::validation},
};

pub async fn provider_directory(
    state: Arc<ServerState>,
    query: ProviderDirectoryQuery,
) -> ApiResult<ProviderDirectoryResponse> {
    let mut conn = match postgres_conn(&state).await {
        Ok(conn) => conn,
        Err(error) => return Err(error),
    };

    let providers = match provider_repository::list_public_providers(
        &mut conn,
        query.q,
        query.service_area,
        validation::directory_limit(query.limit),
    )
    .await
    {
        Ok(providers) => providers,
        Err(error) => return Err(ApiError::from_source(CodeError::DB_QUERY_ERROR, error)),
    };

    let providers = providers
        .into_iter()
        .map(|profile| ProviderDirectoryCardResponse {
            provider_profile_id: profile.provider_profile_id,
            slug: profile.provider_profile_slug,
            display_name: profile.provider_profile_display_name,
            headline: profile.provider_profile_headline,
            service_area: profile.provider_profile_service_area,
            primary_image: None,
        })
        .collect();

    Ok(ProviderDirectoryResponse { providers })
}

pub async fn provider_detail(
    state: Arc<ServerState>,
    slug: String,
) -> ApiResult<ProviderDetailResponse> {
    let slug = match validation::slug_from(Some(slug), "") {
        Ok(slug) => slug,
        Err(error) => return Err(error),
    };

    let mut conn = match postgres_conn(&state).await {
        Ok(conn) => conn,
        Err(error) => return Err(error),
    };

    let profile = match provider_repository::find_public_provider_by_slug(&mut conn, &slug).await {
        Ok(Some(profile)) => profile,
        Ok(None) => return Err(ApiError::new(CodeError::MARKETPLACE_NOT_FOUND)),
        Err(error) => return Err(ApiError::from_source(CodeError::DB_QUERY_ERROR, error)),
    };

    let images = match media_repository::list_public_provider_profile_images(
        &mut conn,
        profile.provider_profile_id,
    )
    .await
    {
        Ok(images) => images.into_iter().map(Into::into).collect(),
        Err(error) => return Err(ApiError::from_source(CodeError::DB_QUERY_ERROR, error)),
    };

    let blog_posts = match provider_repository::list_public_provider_blog_posts(
        &mut conn,
        profile.provider_profile_id,
    )
    .await
    {
        Ok(posts) => posts
            .into_iter()
            .map(|post| {
                let mut response = ProviderBlogPostResponse::from(post);
                response.body = None;
                response.rendered_html = None;
                response
            })
            .collect(),
        Err(error) => return Err(ApiError::from_source(CodeError::DB_QUERY_ERROR, error)),
    };

    Ok(ProviderDetailResponse {
        profile: ProviderProfileResponse::from(profile),
        images,
        blog_posts,
    })
}

pub async fn provider_blog_post(
    state: Arc<ServerState>,
    provider_slug: String,
    post_slug: String,
) -> ApiResult<ProviderBlogPostResponse> {
    let provider_slug = match validation::slug_from(Some(provider_slug), "") {
        Ok(slug) => slug,
        Err(error) => return Err(error),
    };
    let post_slug = match validation::slug_from(Some(post_slug), "") {
        Ok(slug) => slug,
        Err(error) => return Err(error),
    };

    let mut conn = match postgres_conn(&state).await {
        Ok(conn) => conn,
        Err(error) => return Err(error),
    };

    let profile =
        match provider_repository::find_public_provider_by_slug(&mut conn, &provider_slug).await {
            Ok(Some(profile)) => profile,
            Ok(None) => return Err(ApiError::new(CodeError::MARKETPLACE_NOT_FOUND)),
            Err(error) => return Err(ApiError::from_source(CodeError::DB_QUERY_ERROR, error)),
        };

    match provider_repository::find_public_provider_blog_post(
        &mut conn,
        profile.provider_profile_id,
        &post_slug,
    )
    .await
    {
        Ok(Some(post)) => Ok(ProviderBlogPostResponse::from(post)),
        Ok(None) => Err(ApiError::new(CodeError::MARKETPLACE_NOT_FOUND)),
        Err(error) => Err(ApiError::from_source(CodeError::DB_QUERY_ERROR, error)),
    }
}

pub async fn active_banners(
    state: Arc<ServerState>,
    placement: crate::domain::marketplace::enums::BannerPlacement,
) -> ApiResult<BannerListResponse> {
    let mut conn = match postgres_conn(&state).await {
        Ok(conn) => conn,
        Err(error) => return Err(error),
    };

    match admin_repository::list_active_banners(&mut conn, placement, Utc::now()).await {
        Ok(banners) => Ok(BannerListResponse {
            banners: banners.into_iter().map(Into::into).collect(),
        }),
        Err(error) => Err(ApiError::from_source(CodeError::DB_QUERY_ERROR, error)),
    }
}
