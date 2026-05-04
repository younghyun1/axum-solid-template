use std::sync::Arc;

use chrono::{DateTime, Utc};
use diesel_async::{AsyncConnection, AsyncPgConnection};
use tracing::info;

use crate::{
    domain::auth::{
        refresh_session::NewAuthRefreshSession,
        role::RoleType,
        user::{User, UserInfo},
    },
    dto::{
        api_response::ApiResult,
        auth::response::{LogoutResponse, RefreshSessionResponse},
    },
    error::{api_error::ApiError, code_error::CodeError},
    init::state::server_state::ServerState,
    repository::auth::postgres::{
        refresh_session_repository, user_repository, user_role_repository,
    },
    service::auth::datasource::postgres_conn,
    util::{
        auth::jwt::{JwtUserContext, issue_access_token},
        crypto::token::{generate_opaque_token, sha256_hex},
    },
};

#[derive(Debug)]
pub struct IssuedRefreshSession {
    pub refresh_token: String,
    pub refresh_expires_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct IssuedAuthSession<TResponse> {
    pub response: TResponse,
    pub access_token: String,
    pub refresh_token: String,
}

pub async fn issue_refresh_session(
    conn: &mut AsyncPgConnection,
    state: &ServerState,
    user: &User,
    now: DateTime<Utc>,
) -> ApiResult<IssuedRefreshSession> {
    let refresh_token = generate_opaque_token();
    let refresh_token_hash = sha256_hex(&refresh_token);
    let refresh_expires_at =
        match chrono::Duration::from_std(state.server_config.jwt_config.refresh_token_duration) {
            Ok(duration) => now + duration,
            Err(error) => return Err(ApiError::from_source(CodeError::INTERNAL_ERROR, error)),
        };
    let new_session = NewAuthRefreshSession {
        user_id: user.user_id,
        auth_refresh_session_token_hash: refresh_token_hash,
        auth_refresh_session_created_at: now,
        auth_refresh_session_expires_at: refresh_expires_at,
        auth_refresh_session_user_auth_token_version: user.user_auth_token_version,
    };

    match refresh_session_repository::insert_session(conn, new_session).await {
        Ok(_) => Ok(IssuedRefreshSession {
            refresh_token,
            refresh_expires_at,
        }),
        Err(error) => Err(ApiError::from_source(CodeError::DB_INSERT_ERROR, error)),
    }
}

pub async fn refresh_auth_session(
    state: Arc<ServerState>,
    refresh_token: String,
) -> ApiResult<IssuedAuthSession<RefreshSessionResponse>> {
    let mut conn = match postgres_conn(&state).await {
        Ok(conn) => conn,
        Err(error) => return Err(error),
    };
    let refresh_token_hash = sha256_hex(&refresh_token);
    let now = Utc::now();

    let session = {
        let conn = &mut *conn;
        conn.transaction::<_, ApiError, _>(async |conn| {
            let existing_session = match refresh_session_repository::find_by_token_hash(
                &mut *conn,
                &refresh_token_hash,
            )
            .await
            {
                Ok(Some(existing_session)) => existing_session,
                Ok(None) => return Err(ApiError::new(CodeError::REFRESH_SESSION_INVALID)),
                Err(error) => {
                    return Err(ApiError::from_source(CodeError::DB_QUERY_ERROR, error));
                }
            };

            if existing_session.auth_refresh_session_revoked_at.is_some()
                || existing_session.auth_refresh_session_rotated_at.is_some()
            {
                return Err(ApiError::new(CodeError::REFRESH_SESSION_INVALID));
            }
            if existing_session.auth_refresh_session_created_at > now
                || existing_session.auth_refresh_session_expires_at < now
            {
                return Err(ApiError::new(CodeError::REFRESH_SESSION_EXPIRED));
            }

            let user = match user_repository::find_user_by_id(&mut *conn, existing_session.user_id)
                .await
            {
                Ok(Some(user)) => user,
                Ok(None) => {
                    info!(
                        user_id = %existing_session.user_id,
                        refresh_session_id = %existing_session.auth_refresh_session_id,
                        "Refresh session rejected because the user no longer exists"
                    );
                    return Err(ApiError::new(CodeError::REFRESH_SESSION_INVALID));
                }
                Err(error) => {
                    return Err(ApiError::from_source(CodeError::DB_QUERY_ERROR, error));
                }
            };

            if user.user_auth_token_version
                != existing_session.auth_refresh_session_user_auth_token_version
            {
                info!(
                    user_id = %user.user_id,
                    refresh_session_id = %existing_session.auth_refresh_session_id,
                    "Refresh session rejected because the token version is stale"
                );
                return Err(ApiError::new(CodeError::REFRESH_SESSION_INVALID));
            }

            let role_type =
                match user_role_repository::role_for_user(&mut *conn, user.user_id).await {
                    Ok(role_type) => role_type,
                    Err(error) => {
                        return Err(ApiError::from_source(CodeError::DB_QUERY_ERROR, error));
                    }
                };

            let rotated_rows = match refresh_session_repository::rotate_active_session(
                &mut *conn,
                existing_session.auth_refresh_session_id,
                now,
            )
            .await
            {
                Ok(rows) => rows,
                Err(error) => {
                    return Err(ApiError::from_source(CodeError::DB_UPDATE_ERROR, error));
                }
            };
            if rotated_rows != 1 {
                return Err(ApiError::new(CodeError::REFRESH_SESSION_INVALID));
            }

            let refresh_session = match issue_refresh_session(&mut *conn, &state, &user, now).await
            {
                Ok(refresh_session) => refresh_session,
                Err(error) => return Err(error),
            };

            Ok((user, role_type, refresh_session))
        })
        .await
    };

    let (user, role_type, refresh_session) = match session {
        Ok(session) => session,
        Err(error) => return Err(error),
    };
    issue_auth_session(
        &state,
        user,
        role_type,
        refresh_session.refresh_token,
        RefreshSessionResponse::from_parts,
    )
}

pub async fn logout_auth_session(
    state: Arc<ServerState>,
    refresh_token: Option<String>,
) -> ApiResult<LogoutResponse> {
    let token = match refresh_token {
        Some(token) => token,
        None => {
            return Ok(LogoutResponse {
                message: "Logout successful; authentication cookies cleared.",
            });
        }
    };

    let mut conn = match postgres_conn(&state).await {
        Ok(conn) => conn,
        Err(error) => return Err(error),
    };
    let now = Utc::now();
    let token_hash = sha256_hex(&token);
    match refresh_session_repository::revoke_by_token_hash(&mut conn, &token_hash, now).await {
        Ok(_) => Ok(LogoutResponse {
            message: "Logout successful; authentication cookies cleared.",
        }),
        Err(error) => Err(ApiError::from_source(CodeError::DB_UPDATE_ERROR, error)),
    }
}

pub fn issue_auth_session<TResponse, F>(
    state: &ServerState,
    user: User,
    role_type: RoleType,
    refresh_token: String,
    response_builder: F,
) -> ApiResult<IssuedAuthSession<TResponse>>
where
    F: FnOnce(DateTime<Utc>, crate::domain::auth::jwt::AccessTokenClaims, UserInfo) -> TResponse,
{
    let user_info = user_info_from_user(&user);
    let issued = match issue_access_token(
        &state.server_config.jwt_config,
        JwtUserContext { user, role_type },
    ) {
        Ok(issued) => issued,
        Err(error) => return Err(ApiError::from_source(CodeError::JWT_INVALID, error)),
    };
    let response = response_builder(issued.expires_at, issued.claims, user_info);

    Ok(IssuedAuthSession {
        response,
        access_token: issued.token,
        refresh_token,
    })
}

pub fn user_info_from_user(user: &User) -> UserInfo {
    UserInfo {
        user_id: user.user_id,
        user_name: user.user_name.clone(),
        user_email: user.user_email.clone(),
        user_created_at: user.user_created_at,
        user_updated_at: user.user_updated_at,
        user_last_login_at: user.user_last_login_at,
        user_is_email_verified: user.user_is_email_verified,
        user_country: user.user_country,
        user_language: user.user_language,
        user_subdivision: user.user_subdivision,
    }
}

impl RefreshSessionResponse {
    fn from_parts(
        expires_at: DateTime<Utc>,
        claims: crate::domain::auth::jwt::AccessTokenClaims,
        user_info: UserInfo,
    ) -> Self {
        Self {
            expires_at,
            claims,
            user_info,
        }
    }
}
