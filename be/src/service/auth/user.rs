use std::sync::Arc;

use email_address::EmailAddress;

use crate::{
    domain::auth::jwt::AccessTokenClaims,
    dto::{
        api_response::ApiResult,
        auth::{
            request::CheckIfUserExistsRequest,
            response::{CheckIfUserExistsResponse, MeResponse, PublicUserInfoResponse},
        },
    },
    error::{api_error::ApiError, code_error::CodeError},
    init::state::server_state::ServerState,
    repository::auth::postgres::user_repository,
    service::auth::datasource::postgres_conn,
    util::string::validation::normalized_email,
};

pub async fn current_user(
    state: Arc<ServerState>,
    claims: AccessTokenClaims,
) -> ApiResult<MeResponse> {
    let mut conn = match postgres_conn(&state).await {
        Ok(conn) => conn,
        Err(error) => return Err(error),
    };

    let user_info = match user_repository::find_user_info_by_id(&mut conn, claims.user_id).await {
        Ok(Some(user_info)) => user_info,
        Ok(None) => return Err(ApiError::new(CodeError::USER_NOT_FOUND)),
        Err(error) => return Err(ApiError::from_source(CodeError::DB_QUERY_ERROR, error)),
    };

    Ok(MeResponse { user_info, claims })
}

pub async fn check_if_user_exists(
    state: Arc<ServerState>,
    request: CheckIfUserExistsRequest,
) -> ApiResult<CheckIfUserExistsResponse> {
    if !EmailAddress::is_valid(&request.user_email) {
        return Err(ApiError::new(CodeError::EMAIL_INVALID));
    }

    let mut conn = match postgres_conn(&state).await {
        Ok(conn) => conn,
        Err(error) => return Err(error),
    };

    let email = normalized_email(&request.user_email);
    let existing = match user_repository::find_user_id_by_email(&mut conn, &email).await {
        Ok(existing) => existing,
        Err(error) => return Err(ApiError::from_source(CodeError::DB_QUERY_ERROR, error)),
    };

    Ok(CheckIfUserExistsResponse {
        email_exists: existing.is_some(),
    })
}

pub async fn public_user_info(
    state: Arc<ServerState>,
    user_name: String,
) -> ApiResult<PublicUserInfoResponse> {
    let trimmed_user_name = user_name.trim().to_string();
    if trimmed_user_name.is_empty() {
        return Err(ApiError::new(CodeError::USER_NAME_INVALID));
    }

    let mut conn = match postgres_conn(&state).await {
        Ok(conn) => conn,
        Err(error) => return Err(error),
    };

    let user = match user_repository::find_user_by_name(&mut conn, &trimmed_user_name).await {
        Ok(Some(user)) => user,
        Ok(None) => return Err(ApiError::new(CodeError::USER_NOT_FOUND)),
        Err(error) => return Err(ApiError::from_source(CodeError::DB_QUERY_ERROR, error)),
    };

    Ok(PublicUserInfoResponse {
        user_id: user.user_id,
        user_name: user.user_name,
        user_created_at: user.user_created_at,
        user_country: user.user_country,
    })
}
