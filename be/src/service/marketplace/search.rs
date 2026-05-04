use std::sync::Arc;

use crate::{
    domain::auth::jwt::AccessTokenClaims,
    dto::{
        api_response::ApiResult,
        marketplace::{
            request::MarketplaceSearchQuery,
            response::{MarketplaceSearchReindexResponse, MarketplaceSearchResponse},
        },
    },
    error::{api_error::ApiError, code_error::CodeError},
    init::state::server_state::ServerState,
    service::marketplace::{authz, validation},
};

pub async fn search_marketplace(
    state: Arc<ServerState>,
    query: MarketplaceSearchQuery,
) -> ApiResult<MarketplaceSearchResponse> {
    let query_text = match validation::required_text(query.q, 180, "q") {
        Ok(query_text) => query_text,
        Err(error) => return Err(error),
    };
    let limit = validation::search_limit(query.limit);
    let hits = match state
        .marketplace_search_index
        .search(query_text, query.kind, limit)
        .await
    {
        Ok(hits) => hits,
        Err(error) => return Err(ApiError::from_source(CodeError::INTERNAL_ERROR, error)),
    };

    Ok(MarketplaceSearchResponse {
        results: hits.into_iter().map(Into::into).collect(),
    })
}

pub async fn reindex_marketplace_search(
    state: Arc<ServerState>,
    claims: AccessTokenClaims,
) -> ApiResult<MarketplaceSearchReindexResponse> {
    match authz::require_moderator(&claims) {
        Ok(()) => {}
        Err(error) => return Err(error),
    }
    match state.marketplace_search_index.rebuild(&state.db_pool).await {
        Ok(indexed_documents) => Ok(MarketplaceSearchReindexResponse { indexed_documents }),
        Err(error) => Err(ApiError::from_source(CodeError::INTERNAL_ERROR, error)),
    }
}
