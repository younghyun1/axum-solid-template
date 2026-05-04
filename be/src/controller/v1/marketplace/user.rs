use std::sync::Arc;

use axum::{Extension, Json, extract::State};

use crate::{
    dto::{
        api_response::{ApiEnvelope, ApiResponseResult, api_created, api_ok},
        marketplace::{
            request::{CreatePaymentIntentRequest, UpsertUserProfileRequest},
            response::{PaymentIntentListResponse, PaymentIntentResponse, UserProfileResponse},
        },
    },
    init::state::server_state::ServerState,
    middleware::auth::AuthContext,
    service::marketplace::user,
};

#[utoipa::path(
    get,
    path = "/api/v1/marketplace/user/profile",
    tag = "marketplace-user",
    responses((status = 200, description = "Current user marketplace profile", body = ApiEnvelope<UserProfileResponse>))
)]
pub async fn user_get_profile(
    State(state): State<Arc<ServerState>>,
    Extension(auth_context): Extension<AuthContext>,
) -> ApiResponseResult<UserProfileResponse> {
    match user::user_profile(state, auth_context.claims).await {
        Ok(response) => Ok(api_ok(response)),
        Err(error) => Err(error),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/marketplace/user/profile",
    tag = "marketplace-user",
    request_body = UpsertUserProfileRequest,
    responses((status = 200, description = "Updated user marketplace profile", body = ApiEnvelope<UserProfileResponse>))
)]
pub async fn user_upsert_profile(
    State(state): State<Arc<ServerState>>,
    Extension(auth_context): Extension<AuthContext>,
    Json(request): Json<UpsertUserProfileRequest>,
) -> ApiResponseResult<UserProfileResponse> {
    match user::upsert_user_profile(state, auth_context.claims, request).await {
        Ok(response) => Ok(api_ok(response)),
        Err(error) => Err(error),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/marketplace/payments/intents",
    tag = "marketplace-user",
    request_body = CreatePaymentIntentRequest,
    responses((status = 201, description = "Created payment intent", body = ApiEnvelope<PaymentIntentResponse>))
)]
pub async fn user_create_payment_intent(
    State(state): State<Arc<ServerState>>,
    Extension(auth_context): Extension<AuthContext>,
    Json(request): Json<CreatePaymentIntentRequest>,
) -> ApiResponseResult<PaymentIntentResponse> {
    match user::create_payment_intent(state, auth_context.claims, request).await {
        Ok(response) => Ok(api_created(response)),
        Err(error) => Err(error),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/marketplace/payments/intents",
    tag = "marketplace-user",
    responses((status = 200, description = "Current user payment intents", body = ApiEnvelope<PaymentIntentListResponse>))
)]
pub async fn user_list_payment_intents(
    State(state): State<Arc<ServerState>>,
    Extension(auth_context): Extension<AuthContext>,
) -> ApiResponseResult<PaymentIntentListResponse> {
    match user::list_payment_intents(state, auth_context.claims).await {
        Ok(response) => Ok(api_ok(response)),
        Err(error) => Err(error),
    }
}
