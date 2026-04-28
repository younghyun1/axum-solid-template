use std::sync::Arc;

use axum::{
    Extension, Json,
    extract::{Path, Query, State},
};
use chrono::{Duration, Utc};
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;
use email_address::EmailAddress;
use tracing::error;
use uuid::Uuid;
use zeroize::Zeroize;

use crate::{
    domain::auth::{
        role::RoleType,
        user::{
            EmailVerificationToken, NewEmailVerificationToken, NewPasswordResetToken, NewUser,
            PasswordResetToken, User, UserInfo,
        },
        user_role::UserRole,
    },
    dto::{
        api_response::{ApiEnvelope, ApiResponse, ApiResponseResult, ApiTimer},
        auth::{
            request::{
                CheckIfUserExistsRequest, EmailValidationToken, LoginRequest,
                ResetPasswordProcessRequest, ResetPasswordRequest, SignupRequest,
            },
            response::{
                CheckIfUserExistsResponse, IsSuperuserResponse, LoginResponse, LogoutResponse,
                MeResponse, PublicUserInfoResponse, ResetPasswordRequestResponse,
                ResetPasswordResponse, SignupResponse, VerifyEmailResponse,
            },
        },
    },
    error::prelude::*,
    init::state::server_state::ServerState,
    middleware::auth::AuthContext,
    schema::{email_verification_tokens, password_reset_tokens, users},
    util::{
        auth::jwt::{JWT_BEARER_TOKEN_TYPE, JwtUserContext, issue_access_token},
        crypto::password::{hash_password, verify_password},
        email::templates::{email_verification_html, password_reset_html},
        string::validation::{normalized_email, validate_password_form, validate_username},
    },
};

const EMAIL_VERIFICATION_TOKEN_VALID_DURATION: Duration = Duration::days(1);
const PASSWORD_RESET_TOKEN_VALID_DURATION: Duration = Duration::minutes(30);

#[utoipa::path(
    post,
    path = "/api/auth/signup",
    tag = "auth",
    request_body = SignupRequest,
    responses((status = 200, description = "User successfully signed up", body = ApiEnvelope<SignupResponse>))
)]
pub async fn signup(
    State(state): State<Arc<ServerState>>,
    Json(mut request): Json<SignupRequest>,
) -> ApiResponseResult<SignupResponse> {
    let timer = ApiTimer::start();

    if !validate_username(&request.user_name) {
        return Err(ApiError::new(CodeError::USER_NAME_INVALID).with_timer(timer));
    }

    if !EmailAddress::is_valid(&request.user_email) {
        return Err(ApiError::new(CodeError::EMAIL_INVALID).with_timer(timer));
    }

    if !validate_password_form(&request.user_password) {
        return Err(ApiError::new(CodeError::PASSWORD_INVALID).with_timer(timer));
    }

    let user_email = normalized_email(&request.user_email);
    let password = request.user_password.clone();
    let password_hash = match hash_password(password).await {
        Ok(password_hash) => password_hash,
        Err(error) => {
            request.zeroize();
            return Err(
                ApiError::from_source(CodeError::PASSWORD_HASH_ERROR, error).with_timer(timer)
            );
        }
    };

    let mut conn = match state.get_conn().await {
        Ok(conn) => conn,
        Err(error) => {
            request.zeroize();
            return Err(ApiError::from_source(CodeError::DB_POOL_ERROR, error).with_timer(timer));
        }
    };

    let new_user = NewUser {
        user_name: request.user_name.trim().to_string(),
        user_email: user_email.clone(),
        user_password_hash: password_hash,
        user_country: request.user_country,
        user_language: request.user_language,
        user_subdivision: request.user_subdivision,
    };

    let user = match diesel::insert_into(users::table)
        .values(new_user)
        .returning(User::as_returning())
        .get_result::<User>(&mut conn)
        .await
    {
        Ok(user) => user,
        Err(diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::UniqueViolation,
            _,
        )) => {
            request.zeroize();
            return Err(ApiError::new(CodeError::EMAIL_ALREADY_EXISTS).with_timer(timer));
        }
        Err(error) => {
            request.zeroize();
            return Err(ApiError::from_source(CodeError::DB_INSERT_ERROR, error).with_timer(timer));
        }
    };

    match UserRole::insert_for_user(&mut conn, user.user_id, RoleType::User).await {
        Ok(()) => {}
        Err(error) => {
            request.zeroize();
            return Err(ApiError::from_source(CodeError::DB_INSERT_ERROR, error).with_timer(timer));
        }
    }

    let now = Utc::now();
    let email_verification_token = Uuid::now_v7();
    let verify_by = now + EMAIL_VERIFICATION_TOKEN_VALID_DURATION;
    let new_token = NewEmailVerificationToken {
        user_id: user.user_id,
        email_verification_token,
        email_verification_token_expires_at: verify_by,
        email_verification_token_created_at: now,
    };

    match diesel::insert_into(email_verification_tokens::table)
        .values(new_token)
        .execute(&mut conn)
        .await
    {
        Ok(_) => {}
        Err(error) => {
            request.zeroize();
            return Err(ApiError::from_source(CodeError::DB_INSERT_ERROR, error).with_timer(timer));
        }
    }

    drop(conn);
    request.zeroize();
    queue_verification_email(
        state,
        user.user_email.clone(),
        email_verification_token,
        verify_by,
    );

    Ok(api_ok_timed(
        SignupResponse {
            user_id: user.user_id,
            user_name: user.user_name,
            user_email: user.user_email,
            verify_by,
        },
        timer,
    ))
}

#[utoipa::path(
    post,
    path = "/api/auth/login",
    tag = "auth",
    request_body = LoginRequest,
    responses((status = 200, description = "Login successful", body = ApiEnvelope<LoginResponse>))
)]
pub async fn login(
    State(state): State<Arc<ServerState>>,
    Json(mut request): Json<LoginRequest>,
) -> ApiResponseResult<LoginResponse> {
    let timer = ApiTimer::start();

    if !EmailAddress::is_valid(&request.user_email) {
        return Err(ApiError::new(CodeError::EMAIL_INVALID).with_timer(timer));
    }

    if !validate_password_form(&request.user_password) {
        return Err(ApiError::new(CodeError::PASSWORD_INVALID).with_timer(timer));
    }

    let mut conn = match state.get_conn().await {
        Ok(conn) => conn,
        Err(error) => {
            request.zeroize();
            return Err(ApiError::from_source(CodeError::DB_POOL_ERROR, error).with_timer(timer));
        }
    };

    let user_email = normalized_email(&request.user_email);
    let user = match users::table
        .filter(users::user_email.eq(&user_email))
        .select(User::as_select())
        .first::<User>(&mut conn)
        .await
        .optional()
    {
        Ok(Some(user)) => user,
        Ok(None) => {
            request.zeroize();
            return Err(ApiError::new(CodeError::USER_NOT_FOUND).with_timer(timer));
        }
        Err(error) => {
            request.zeroize();
            return Err(ApiError::from_source(CodeError::DB_QUERY_ERROR, error).with_timer(timer));
        }
    };

    let password_matches = match verify_password(
        request.user_password.clone(),
        user.user_password_hash.clone(),
    )
    .await
    {
        Ok(password_matches) => password_matches,
        Err(error) => {
            request.zeroize();
            return Err(
                ApiError::from_source(CodeError::PASSWORD_VERIFY_ERROR, error).with_timer(timer),
            );
        }
    };

    request.zeroize();
    if !password_matches {
        return Err(ApiError::new(CodeError::WRONG_PASSWORD).with_timer(timer));
    }

    if !user.user_is_email_verified {
        return Err(ApiError::new(CodeError::EMAIL_NOT_VERIFIED).with_timer(timer));
    }

    let role_type = match UserRole::role_for_user(&mut conn, user.user_id).await {
        Ok(role_type) => role_type,
        Err(error) => {
            return Err(ApiError::from_source(CodeError::DB_QUERY_ERROR, error).with_timer(timer));
        }
    };
    let role_name = match UserRole::role_name_for_user(&mut conn, user.user_id).await {
        Ok(role_name) => role_name,
        Err(error) => {
            return Err(ApiError::from_source(CodeError::DB_QUERY_ERROR, error).with_timer(timer));
        }
    };

    match diesel::update(users::table.filter(users::user_id.eq(user.user_id)))
        .set((
            users::user_last_login_at.eq(Utc::now()),
            users::user_updated_at.eq(Utc::now()),
        ))
        .execute(&mut conn)
        .await
    {
        Ok(_) => {}
        Err(error) => {
            error!(error = %error, user_id = %user.user_id, "Failed to update user last login");
        }
    }

    let issued = match issue_access_token(
        &state.server_config.jwt_config,
        JwtUserContext {
            user,
            role_type,
            role_name,
        },
    ) {
        Ok(issued) => issued,
        Err(error) => {
            return Err(ApiError::from_source(CodeError::JWT_INVALID, error).with_timer(timer));
        }
    };

    Ok(api_ok_timed(
        LoginResponse {
            access_token: issued.token,
            token_type: JWT_BEARER_TOKEN_TYPE,
            expires_at: issued.expires_at,
            claims: issued.claims,
        },
        timer,
    ))
}

#[utoipa::path(
    get,
    path = "/api/auth/me",
    tag = "auth",
    responses((status = 200, description = "Current user", body = ApiEnvelope<MeResponse>))
)]
pub async fn me(
    Extension(auth_context): Extension<AuthContext>,
    State(state): State<Arc<ServerState>>,
) -> ApiResponseResult<MeResponse> {
    let timer = ApiTimer::start();
    let mut conn = match state.get_conn().await {
        Ok(conn) => conn,
        Err(error) => {
            return Err(ApiError::from_source(CodeError::DB_POOL_ERROR, error).with_timer(timer));
        }
    };

    let user_info = match users::table
        .filter(users::user_id.eq(auth_context.claims.user_id))
        .select(UserInfo::as_select())
        .first::<UserInfo>(&mut conn)
        .await
        .optional()
    {
        Ok(Some(user_info)) => user_info,
        Ok(None) => return Err(ApiError::new(CodeError::USER_NOT_FOUND).with_timer(timer)),
        Err(error) => {
            return Err(ApiError::from_source(CodeError::DB_QUERY_ERROR, error).with_timer(timer));
        }
    };

    Ok(api_ok_timed(
        MeResponse {
            user_info,
            claims: auth_context.claims,
        },
        timer,
    ))
}

#[utoipa::path(
    post,
    path = "/api/auth/logout",
    tag = "auth",
    responses((status = 200, description = "Logout successful", body = ApiEnvelope<LogoutResponse>))
)]
pub async fn logout() -> ApiResponse<LogoutResponse> {
    api_ok(LogoutResponse {
        message: "Logout successful; discard the JWT on the client.",
    })
}

#[utoipa::path(
    get,
    path = "/api/auth/is-superuser",
    tag = "auth",
    responses((status = 200, description = "Current superuser status", body = ApiEnvelope<IsSuperuserResponse>))
)]
pub async fn is_superuser(
    Extension(auth_context): Extension<AuthContext>,
) -> ApiResponse<IsSuperuserResponse> {
    api_ok(IsSuperuserResponse {
        is_superuser: auth_context.claims.role_type.is_superuser(),
    })
}

#[utoipa::path(
    post,
    path = "/api/auth/check-if-user-exists",
    tag = "auth",
    request_body = CheckIfUserExistsRequest,
    responses((status = 200, description = "Email existence", body = ApiEnvelope<CheckIfUserExistsResponse>))
)]
pub async fn check_if_user_exists(
    State(state): State<Arc<ServerState>>,
    Json(request): Json<CheckIfUserExistsRequest>,
) -> ApiResponseResult<CheckIfUserExistsResponse> {
    let timer = ApiTimer::start();
    if !EmailAddress::is_valid(&request.user_email) {
        return Err(ApiError::new(CodeError::EMAIL_INVALID).with_timer(timer));
    }

    let mut conn = match state.get_conn().await {
        Ok(conn) => conn,
        Err(error) => {
            return Err(ApiError::from_source(CodeError::DB_POOL_ERROR, error).with_timer(timer));
        }
    };

    let email = normalized_email(&request.user_email);
    let existing = match users::table
        .filter(users::user_email.eq(&email))
        .select(users::user_id)
        .first::<Uuid>(&mut conn)
        .await
        .optional()
    {
        Ok(existing) => existing,
        Err(error) => {
            return Err(ApiError::from_source(CodeError::DB_QUERY_ERROR, error).with_timer(timer));
        }
    };

    Ok(api_ok_timed(
        CheckIfUserExistsResponse {
            email_exists: existing.is_some(),
        },
        timer,
    ))
}

#[utoipa::path(
    post,
    path = "/api/auth/reset-password-request",
    tag = "auth",
    request_body = ResetPasswordRequest,
    responses((status = 200, description = "Password reset request processed", body = ApiEnvelope<ResetPasswordRequestResponse>))
)]
pub async fn reset_password_request(
    State(state): State<Arc<ServerState>>,
    Json(request): Json<ResetPasswordRequest>,
) -> ApiResponseResult<ResetPasswordRequestResponse> {
    let timer = ApiTimer::start();
    if !EmailAddress::is_valid(&request.user_email) {
        return Err(ApiError::new(CodeError::EMAIL_INVALID).with_timer(timer));
    }

    let mut conn = match state.get_conn().await {
        Ok(conn) => conn,
        Err(error) => {
            return Err(ApiError::from_source(CodeError::DB_POOL_ERROR, error).with_timer(timer));
        }
    };

    let email = normalized_email(&request.user_email);
    let user = match users::table
        .filter(users::user_email.eq(&email))
        .select(User::as_select())
        .first::<User>(&mut conn)
        .await
        .optional()
    {
        Ok(Some(user)) => user,
        Ok(None) => return Err(ApiError::new(CodeError::USER_NOT_FOUND).with_timer(timer)),
        Err(error) => {
            return Err(ApiError::from_source(CodeError::DB_QUERY_ERROR, error).with_timer(timer));
        }
    };

    let now = Utc::now();
    let password_reset_token = Uuid::now_v7();
    let verify_by = now + PASSWORD_RESET_TOKEN_VALID_DURATION;
    let new_token = NewPasswordResetToken {
        user_id: user.user_id,
        password_reset_token,
        password_reset_token_expires_at: verify_by,
        password_reset_token_created_at: now,
    };

    match diesel::insert_into(password_reset_tokens::table)
        .values(new_token)
        .execute(&mut conn)
        .await
    {
        Ok(_) => {}
        Err(error) => {
            return Err(ApiError::from_source(CodeError::DB_INSERT_ERROR, error).with_timer(timer));
        }
    }

    drop(conn);
    queue_password_reset_email(
        state,
        user.user_email.clone(),
        password_reset_token,
        verify_by,
    );

    Ok(api_ok_timed(
        ResetPasswordRequestResponse {
            user_email: user.user_email,
            verify_by,
            delivery_queued: true,
        },
        timer,
    ))
}

#[utoipa::path(
    post,
    path = "/api/auth/reset-password",
    tag = "auth",
    request_body = ResetPasswordProcessRequest,
    responses((status = 200, description = "Password reset", body = ApiEnvelope<ResetPasswordResponse>))
)]
pub async fn reset_password(
    State(state): State<Arc<ServerState>>,
    Json(mut request): Json<ResetPasswordProcessRequest>,
) -> ApiResponseResult<ResetPasswordResponse> {
    let timer = ApiTimer::start();
    if !validate_password_form(&request.new_password) {
        return Err(ApiError::new(CodeError::PASSWORD_INVALID).with_timer(timer));
    }

    let mut conn = match state.get_conn().await {
        Ok(conn) => conn,
        Err(error) => {
            request.zeroize();
            return Err(ApiError::from_source(CodeError::DB_POOL_ERROR, error).with_timer(timer));
        }
    };

    let now = Utc::now();
    let reset_token = match password_reset_tokens::table
        .filter(password_reset_tokens::password_reset_token.eq(request.password_reset_token))
        .select(PasswordResetToken::as_select())
        .first::<PasswordResetToken>(&mut conn)
        .await
        .optional()
    {
        Ok(Some(reset_token)) => reset_token,
        Ok(None) => {
            request.zeroize();
            return Err(ApiError::new(CodeError::PASSWORD_RESET_TOKEN_INVALID).with_timer(timer));
        }
        Err(error) => {
            request.zeroize();
            return Err(ApiError::from_source(CodeError::DB_QUERY_ERROR, error).with_timer(timer));
        }
    };

    if reset_token.password_reset_token_used_at.is_some() {
        request.zeroize();
        return Err(ApiError::new(CodeError::PASSWORD_RESET_TOKEN_ALREADY_USED).with_timer(timer));
    }
    if reset_token.password_reset_token_created_at > now
        || reset_token.password_reset_token_expires_at < now
    {
        request.zeroize();
        return Err(ApiError::new(CodeError::PASSWORD_RESET_TOKEN_EXPIRED).with_timer(timer));
    }

    let new_password_hash = match hash_password(request.new_password.clone()).await {
        Ok(new_password_hash) => new_password_hash,
        Err(error) => {
            request.zeroize();
            return Err(
                ApiError::from_source(CodeError::PASSWORD_HASH_ERROR, error).with_timer(timer)
            );
        }
    };
    request.zeroize();

    let user = match diesel::update(users::table.filter(users::user_id.eq(reset_token.user_id)))
        .set((
            users::user_password_hash.eq(new_password_hash),
            users::user_password_changed_at.eq(now),
            users::user_updated_at.eq(now),
            users::user_auth_token_version.eq(users::user_auth_token_version + 1),
        ))
        .returning(User::as_returning())
        .get_result::<User>(&mut conn)
        .await
    {
        Ok(user) => user,
        Err(error) => {
            return Err(ApiError::from_source(CodeError::DB_UPDATE_ERROR, error).with_timer(timer));
        }
    };

    match diesel::update(password_reset_tokens::table.filter(
        password_reset_tokens::password_reset_token_id.eq(reset_token.password_reset_token_id),
    ))
    .set(password_reset_tokens::password_reset_token_used_at.eq(now))
    .execute(&mut conn)
    .await
    {
        Ok(_) => {}
        Err(error) => {
            error!(
                error = %error,
                password_reset_token_id = %reset_token.password_reset_token_id,
                "Failed to mark password reset token as used"
            );
        }
    }

    Ok(api_ok_timed(
        ResetPasswordResponse {
            user_id: user.user_id,
            user_name: user.user_name,
            user_email: user.user_email,
            user_updated_at: user.user_updated_at,
        },
        timer,
    ))
}

#[utoipa::path(
    get,
    path = "/api/auth/verify-user-email",
    tag = "auth",
    params(("email_validation_token_id" = Uuid, Query, description = "Email validation token")),
    responses((status = 200, description = "Email verified", body = ApiEnvelope<VerifyEmailResponse>))
)]
pub async fn verify_user_email(
    State(state): State<Arc<ServerState>>,
    Query(token): Query<EmailValidationToken>,
) -> ApiResponseResult<VerifyEmailResponse> {
    let timer = ApiTimer::start();
    let mut conn = match state.get_conn().await {
        Ok(conn) => conn,
        Err(error) => {
            return Err(ApiError::from_source(CodeError::DB_POOL_ERROR, error).with_timer(timer));
        }
    };

    let now = Utc::now();
    let verification_token = match email_verification_tokens::table
        .filter(
            email_verification_tokens::email_verification_token.eq(token.email_validation_token_id),
        )
        .select(EmailVerificationToken::as_select())
        .first::<EmailVerificationToken>(&mut conn)
        .await
        .optional()
    {
        Ok(Some(verification_token)) => verification_token,
        Ok(None) => {
            return Err(
                ApiError::new(CodeError::EMAIL_VERIFICATION_TOKEN_INVALID).with_timer(timer)
            );
        }
        Err(error) => {
            return Err(ApiError::from_source(CodeError::DB_QUERY_ERROR, error).with_timer(timer));
        }
    };

    if verification_token
        .email_verification_token_used_at
        .is_some()
    {
        return Err(
            ApiError::new(CodeError::EMAIL_VERIFICATION_TOKEN_ALREADY_USED).with_timer(timer),
        );
    }
    if verification_token.email_verification_token_created_at > now
        || verification_token.email_verification_token_expires_at < now
    {
        return Err(ApiError::new(CodeError::EMAIL_VERIFICATION_TOKEN_EXPIRED).with_timer(timer));
    }

    let user =
        match diesel::update(users::table.filter(users::user_id.eq(verification_token.user_id)))
            .set((
                users::user_is_email_verified.eq(true),
                users::user_updated_at.eq(now),
            ))
            .returning(User::as_returning())
            .get_result::<User>(&mut conn)
            .await
        {
            Ok(user) => user,
            Err(error) => {
                return Err(
                    ApiError::from_source(CodeError::DB_UPDATE_ERROR, error).with_timer(timer)
                );
            }
        };

    match diesel::update(
        email_verification_tokens::table.filter(
            email_verification_tokens::email_verification_token_id
                .eq(verification_token.email_verification_token_id),
        ),
    )
    .set(email_verification_tokens::email_verification_token_used_at.eq(now))
    .execute(&mut conn)
    .await
    {
        Ok(_) => {}
        Err(error) => {
            error!(
                error = %error,
                email_verification_token_id = %verification_token.email_verification_token_id,
                "Failed to mark email verification token as used"
            );
        }
    }

    Ok(api_ok_timed(
        VerifyEmailResponse {
            user_id: user.user_id,
            user_email: user.user_email,
            verified_at: now,
        },
        timer,
    ))
}

#[utoipa::path(
    get,
    path = "/api/users/{user_name}",
    tag = "user",
    params(("user_name" = String, Path, description = "Public username")),
    responses((status = 200, description = "Public user information", body = ApiEnvelope<PublicUserInfoResponse>))
)]
pub async fn get_user_info(
    State(state): State<Arc<ServerState>>,
    Path(user_name): Path<String>,
) -> ApiResponseResult<PublicUserInfoResponse> {
    let timer = ApiTimer::start();
    let trimmed_user_name = user_name.trim().to_string();
    if trimmed_user_name.is_empty() {
        return Err(ApiError::new(CodeError::USER_NAME_INVALID).with_timer(timer));
    }

    let mut conn = match state.get_conn().await {
        Ok(conn) => conn,
        Err(error) => {
            return Err(ApiError::from_source(CodeError::DB_POOL_ERROR, error).with_timer(timer));
        }
    };

    let user = match users::table
        .filter(users::user_name.eq(&trimmed_user_name))
        .select(User::as_select())
        .first::<User>(&mut conn)
        .await
        .optional()
    {
        Ok(Some(user)) => user,
        Ok(None) => return Err(ApiError::new(CodeError::USER_NOT_FOUND).with_timer(timer)),
        Err(error) => {
            return Err(ApiError::from_source(CodeError::DB_QUERY_ERROR, error).with_timer(timer));
        }
    };

    Ok(api_ok_timed(
        PublicUserInfoResponse {
            user_id: user.user_id,
            user_name: user.user_name,
            user_created_at: user.user_created_at,
            user_country: user.user_country,
        },
        timer,
    ))
}

fn queue_verification_email(
    state: Arc<ServerState>,
    user_email: String,
    token: Uuid,
    verify_by: chrono::DateTime<Utc>,
) {
    tokio::spawn(async move {
        let html = email_verification_html(token, verify_by);
        match state
            .mail_sender
            .send_html(&user_email, "Verify your email", html)
            .await
        {
            Ok(()) => {}
            Err(error) => {
                error!(error = %error, user_email = %user_email, "Failed to send verification email");
            }
        }
    });
}

fn queue_password_reset_email(
    state: Arc<ServerState>,
    user_email: String,
    token: Uuid,
    verify_by: chrono::DateTime<Utc>,
) {
    tokio::spawn(async move {
        let html = password_reset_html(token, verify_by);
        match state
            .mail_sender
            .send_html(&user_email, "Reset your password", html)
            .await
        {
            Ok(()) => {}
            Err(error) => {
                error!(error = %error, user_email = %user_email, "Failed to send password reset email");
            }
        }
    });
}
