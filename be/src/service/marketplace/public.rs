use std::sync::Arc;

use chrono::Utc;
use serde::{Serialize, de::DeserializeOwned};
use tracing::warn;

use crate::{
    dto::{
        api_response::ApiResult,
        marketplace::{
            request::ProviderDirectoryQuery,
            response::{
                BannerListResponse, ProviderBlogPostResponse, ProviderDetailResponse,
                ProviderDirectoryCardResponse, ProviderDirectoryResponse, ProviderProfileResponse,
                ProviderSubdivisionResponse,
            },
        },
    },
    error::{api_error::ApiError, code_error::CodeError},
    init::state::server_state::ServerState,
    repository::marketplace::postgres::{
        admin_repository, media_repository,
        provider_repository::{self, SubdivisionWithCountry},
    },
    service::{auth::datasource::postgres_conn, marketplace::validation},
};

fn subdivision_response(entry: &SubdivisionWithCountry) -> ProviderSubdivisionResponse {
    ProviderSubdivisionResponse {
        subdivision_id: entry.subdivision.subdivision_id,
        country_code: entry.subdivision.country_code,
        country_alpha2: entry.country_alpha2.clone(),
        subdivision_code: entry.subdivision.subdivision_code.clone(),
        subdivision_name: entry.subdivision.subdivision_name.clone(),
        subdivision_type: entry.subdivision.subdivision_type.clone(),
    }
}

pub async fn provider_directory(
    state: Arc<ServerState>,
    query: ProviderDirectoryQuery,
) -> ApiResult<ProviderDirectoryResponse> {
    let limit = validation::directory_limit(query.limit);
    let cache_key = crate::init::state::cache::marketplace::key::provider_directory(
        query.q.as_deref(),
        query.subdivision_id,
        query.subdivision_code.as_deref(),
        limit,
    );
    if let Some(response) =
        read_cached_response::<ProviderDirectoryResponse>(&state, &cache_key).await
    {
        return Ok(response);
    }

    let mut conn = match postgres_conn(&state).await {
        Ok(conn) => conn,
        Err(error) => return Err(error),
    };

    let resolved_subdivision_id = match query.subdivision_id {
        Some(id) => Some(id),
        None => match query.subdivision_code.as_deref() {
            Some(code) if !code.trim().is_empty() => {
                match provider_repository::find_subdivision_id_by_code(&mut conn, code.trim()).await
                {
                    Ok(Some(id)) => Some(id),
                    Ok(None) => {
                        let response = ProviderDirectoryResponse {
                            providers: Vec::new(),
                        };
                        write_cached_response(&state, cache_key, &response).await;
                        return Ok(response);
                    }
                    Err(error) => {
                        return Err(ApiError::from_source(CodeError::DB_QUERY_ERROR, error));
                    }
                }
            }
            _ => None,
        },
    };

    let providers = match provider_repository::list_public_providers(
        &mut conn,
        query.q,
        resolved_subdivision_id,
        limit,
    )
    .await
    {
        Ok(providers) => providers,
        Err(error) => return Err(ApiError::from_source(CodeError::DB_QUERY_ERROR, error)),
    };

    let subdivision_ids: Vec<i32> = providers
        .iter()
        .filter_map(|profile| profile.provider_profile_subdivision_id)
        .collect();
    let subdivision_lookup = match provider_repository::load_subdivisions_with_country(
        &mut conn,
        &subdivision_ids,
    )
    .await
    {
        Ok(map) => map,
        Err(error) => return Err(ApiError::from_source(CodeError::DB_QUERY_ERROR, error)),
    };

    let providers = providers
        .into_iter()
        .map(|profile| {
            let subdivision = profile
                .provider_profile_subdivision_id
                .and_then(|id| subdivision_lookup.get(&id))
                .map(subdivision_response);
            ProviderDirectoryCardResponse {
                provider_profile_id: profile.provider_profile_id,
                slug: profile.provider_profile_slug,
                display_name: profile.provider_profile_display_name,
                headline: profile.provider_profile_headline,
                subdivision,
                primary_image: None,
            }
        })
        .collect();

    let response = ProviderDirectoryResponse { providers };
    write_cached_response(&state, cache_key, &response).await;
    Ok(response)
}

pub async fn provider_detail(
    state: Arc<ServerState>,
    slug: String,
) -> ApiResult<ProviderDetailResponse> {
    let slug = match validation::slug_from(Some(slug), "") {
        Ok(slug) => slug,
        Err(error) => return Err(error),
    };
    let cache_key = crate::init::state::cache::marketplace::key::provider_detail(&slug);
    if let Some(response) = read_cached_response::<ProviderDetailResponse>(&state, &cache_key).await
    {
        return Ok(response);
    }

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

    let subdivision_ids: Vec<i32> = profile
        .provider_profile_subdivision_id
        .into_iter()
        .collect();
    let subdivision_lookup = match provider_repository::load_subdivisions_with_country(
        &mut conn,
        &subdivision_ids,
    )
    .await
    {
        Ok(map) => map,
        Err(error) => return Err(ApiError::from_source(CodeError::DB_QUERY_ERROR, error)),
    };
    let subdivision = profile
        .provider_profile_subdivision_id
        .and_then(|id| subdivision_lookup.get(&id))
        .map(subdivision_response);

    let mut profile_response = ProviderProfileResponse::from(profile);
    profile_response.subdivision = subdivision;
    let response = ProviderDetailResponse {
        profile: profile_response,
        images,
        blog_posts,
    };
    write_cached_response(&state, cache_key, &response).await;
    Ok(response)
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
    let cache_key =
        crate::init::state::cache::marketplace::key::provider_blog_post(&provider_slug, &post_slug);
    if let Some(response) =
        read_cached_response::<ProviderBlogPostResponse>(&state, &cache_key).await
    {
        return Ok(response);
    }

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
        Ok(Some(post)) => {
            let response = ProviderBlogPostResponse::from(post);
            write_cached_response(&state, cache_key, &response).await;
            Ok(response)
        }
        Ok(None) => Err(ApiError::new(CodeError::MARKETPLACE_NOT_FOUND)),
        Err(error) => Err(ApiError::from_source(CodeError::DB_QUERY_ERROR, error)),
    }
}

pub async fn active_banners(
    state: Arc<ServerState>,
    placement: crate::domain::marketplace::enums::BannerPlacement,
) -> ApiResult<BannerListResponse> {
    let cache_key = crate::init::state::cache::marketplace::key::active_banners(placement);
    if let Some(response) = read_cached_response::<BannerListResponse>(&state, &cache_key).await {
        return Ok(response);
    }

    let mut conn = match postgres_conn(&state).await {
        Ok(conn) => conn,
        Err(error) => return Err(error),
    };

    match admin_repository::list_active_banners(&mut conn, placement, Utc::now()).await {
        Ok(banners) => {
            let response = BannerListResponse {
                banners: banners.into_iter().map(Into::into).collect(),
            };
            write_cached_response(&state, cache_key, &response).await;
            Ok(response)
        }
        Err(error) => Err(ApiError::from_source(CodeError::DB_QUERY_ERROR, error)),
    }
}

async fn read_cached_response<T>(state: &ServerState, cache_key: &str) -> Option<T>
where
    T: DeserializeOwned,
{
    let json = match state.marketplace_public_cache.get_json(cache_key).await {
        Some(json) => json,
        None => return None,
    };
    match serde_json::from_str::<T>(&json) {
        Ok(response) => Some(response),
        Err(error) => {
            warn!(
                cache_key = %cache_key,
                error = %error,
                "Ignoring invalid marketplace cache entry"
            );
            None
        }
    }
}

async fn write_cached_response<T>(state: &ServerState, cache_key: String, response: &T)
where
    T: Serialize,
{
    let json = match serde_json::to_string(response) {
        Ok(json) => json,
        Err(error) => {
            warn!(
                cache_key = %cache_key,
                error = %error,
                "Failed to serialize marketplace cache entry"
            );
            return;
        }
    };
    match state
        .marketplace_public_cache
        .put_json(cache_key.clone(), json)
        .await
    {
        Ok(()) => {}
        Err(error) => {
            warn!(
                cache_key = %cache_key,
                error = %error,
                "Failed to persist marketplace cache entry"
            );
        }
    }
}
