use std::sync::Arc;

use chrono::Utc;
use uuid::Uuid;

use crate::{
    domain::auth::jwt::AccessTokenClaims,
    dto::{
        api_response::ApiResult,
        marketplace::{
            request::ModerationDecisionRequest,
            response::{ProviderBlogPostResponse, ProviderProfileResponse},
        },
    },
    error::{api_error::ApiError, code_error::CodeError},
    init::state::server_state::ServerState,
    repository::marketplace::postgres::moderation_repository,
    service::{
        auth::datasource::postgres_conn,
        marketplace::{authz, cache, indexing},
    },
};

pub async fn moderate_provider_profile(
    state: Arc<ServerState>,
    claims: AccessTokenClaims,
    provider_profile_id: Uuid,
    request: ModerationDecisionRequest,
) -> ApiResult<ProviderProfileResponse> {
    match authz::require_moderator(&claims) {
        Ok(()) => {}
        Err(error) => return Err(error),
    }
    let mut conn = match postgres_conn(&state).await {
        Ok(conn) => conn,
        Err(error) => return Err(error),
    };
    match moderation_repository::update_provider_profile_moderation(
        &mut conn,
        provider_profile_id,
        request.moderation_status,
        Utc::now(),
    )
    .await
    {
        Ok(profile) => {
            cache::clear_public_cache(&state, "provider_profile_moderation").await;
            indexing::rebuild_search_index(&state, "provider_profile_moderation").await;
            Ok(ProviderProfileResponse::from(profile))
        }
        Err(error) => Err(ApiError::from_source(CodeError::DB_UPDATE_ERROR, error)),
    }
}

pub async fn moderate_provider_blog_post(
    state: Arc<ServerState>,
    claims: AccessTokenClaims,
    provider_blog_post_id: Uuid,
    request: ModerationDecisionRequest,
) -> ApiResult<ProviderBlogPostResponse> {
    match authz::require_moderator(&claims) {
        Ok(()) => {}
        Err(error) => return Err(error),
    }
    let mut conn = match postgres_conn(&state).await {
        Ok(conn) => conn,
        Err(error) => return Err(error),
    };
    match moderation_repository::update_provider_blog_post_moderation(
        &mut conn,
        provider_blog_post_id,
        request.moderation_status,
        Utc::now(),
    )
    .await
    {
        Ok(post) => {
            cache::clear_public_cache(&state, "provider_blog_post_moderation").await;
            indexing::rebuild_search_index(&state, "provider_blog_post_moderation").await;
            Ok(ProviderBlogPostResponse::from(post))
        }
        Err(error) => Err(ApiError::from_source(CodeError::DB_UPDATE_ERROR, error)),
    }
}
