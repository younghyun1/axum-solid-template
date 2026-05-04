use tracing::info;

use crate::{
    domain::auth::jwt::AccessTokenClaims,
    error::{api_error::ApiError, code_error::CodeError},
    init::state::server_state::ServerState,
    repository::auth::postgres::{user_repository, user_role_repository},
    service::auth::datasource::postgres_conn,
};

/// Confirms the JWT still points at the current database user and role.
///
/// # Arguments
/// * `state` - Shared state used to open a database connection.
/// * `claims` - Decoded JWT claims to validate against persisted auth state.
///
/// # Returns
/// `Ok(())` when the token still matches the current user record; otherwise an API error.
pub async fn validate_claims_against_current_user(
    state: &ServerState,
    claims: &AccessTokenClaims,
) -> Result<(), ApiError> {
    let mut conn = match postgres_conn(state).await {
        Ok(conn) => conn,
        Err(error) => return Err(error),
    };

    let user = match user_repository::find_user_by_id(&mut conn, claims.user_id).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            info!(
                user_id = %claims.user_id,
                jwt_id = %claims.jti,
                "JWT access token rejected because the user no longer exists"
            );
            return Err(ApiError::new(CodeError::JWT_INVALID));
        }
        Err(error) => return Err(ApiError::from_source(CodeError::DB_QUERY_ERROR, error)),
    };

    if user.user_auth_token_version != claims.user_auth_token_version {
        info!(
            user_id = %claims.user_id,
            jwt_id = %claims.jti,
            claim_token_version = claims.user_auth_token_version,
            current_token_version = user.user_auth_token_version,
            "JWT access token rejected because the token version is stale"
        );
        return Err(ApiError::new(CodeError::JWT_INVALID));
    }

    let role_type = match user_role_repository::role_for_user(&mut conn, claims.user_id).await {
        Ok(role_type) => role_type,
        Err(error) => return Err(ApiError::from_source(CodeError::DB_QUERY_ERROR, error)),
    };

    if role_type != claims.role_type {
        info!(
            user_id = %claims.user_id,
            jwt_id = %claims.jti,
            claim_role = claims.role_type.as_str(),
            current_role = role_type.as_str(),
            "JWT access token rejected because the role changed"
        );
        return Err(ApiError::new(CodeError::JWT_INVALID));
    }

    Ok(())
}
