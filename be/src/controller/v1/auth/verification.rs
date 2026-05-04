use std::{net::SocketAddr, sync::Arc};

use axum::{
    Json,
    extract::{ConnectInfo, Query, State},
    http::HeaderMap,
};

use crate::{
    dto::{
        api_response::{ApiEnvelope, ApiResponseResult},
        auth::{
            request::{EmailValidationToken, VerifyEmailChallengeRequest},
            response::{EmailVerificationChallengeResponse, VerifyEmailResponse},
        },
    },
    init::state::server_state::ServerState,
    service::auth::verification::{
        issue::issue_email_verification_challenge,
        submit::verify_user_email as verify_user_email_service,
    },
};

use super::support::{response_from_result, user_agent};

#[utoipa::path(
    get,
    path = "/api/v1/auth/email-verification/challenge",
    tag = "auth",
    params(("email_validation_token_id" = uuid::Uuid, Query, description = "Email validation token")),
    responses((status = 200, description = "Email verification challenge", body = ApiEnvelope<EmailVerificationChallengeResponse>))
)]
pub async fn email_verification_challenge(
    State(state): State<Arc<ServerState>>,
    Query(token): Query<EmailValidationToken>,
    ConnectInfo(client_addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
) -> ApiResponseResult<EmailVerificationChallengeResponse> {
    response_from_result(
        issue_email_verification_challenge(state, token, Some(client_addr), user_agent(&headers))
            .await,
    )
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/verify-user-email",
    tag = "auth",
    request_body = VerifyEmailChallengeRequest,
    responses((status = 200, description = "Email verified", body = ApiEnvelope<VerifyEmailResponse>))
)]
pub async fn verify_user_email(
    State(state): State<Arc<ServerState>>,
    Json(request): Json<VerifyEmailChallengeRequest>,
) -> ApiResponseResult<VerifyEmailResponse> {
    response_from_result(verify_user_email_service(state, request).await)
}
