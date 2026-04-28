use std::sync::Arc;

use crate::{
    domain::auth::{
        jwt::AccessTokenClaims,
        value::{UserEmail, UserName},
    },
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
    let user_email = match UserEmail::try_new(request.user_email) {
        Ok(user_email) => user_email,
        Err(_) => return Err(ApiError::new(CodeError::EMAIL_INVALID)),
    };

    let mut conn = match postgres_conn(&state).await {
        Ok(conn) => conn,
        Err(error) => return Err(error),
    };

    let email = user_email.into_inner();
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
    let user_name = match UserName::try_new(user_name) {
        Ok(user_name) => user_name,
        Err(_) => return Err(ApiError::new(CodeError::USER_NAME_INVALID)),
    };

    let mut conn = match postgres_conn(&state).await {
        Ok(conn) => conn,
        Err(error) => return Err(error),
    };

    let user_name = user_name.into_inner();
    let user = match user_repository::find_user_by_name(&mut conn, &user_name).await {
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
