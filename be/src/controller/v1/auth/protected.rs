use std::sync::Arc;

use axum::{Extension, extract::State};

use crate::{
    dto::{
        api_response::{ApiEnvelope, ApiResponseResult},
        auth::response::MeResponse,
    },
    init::state::server_state::ServerState,
    middleware::auth::AuthContext,
    service::auth::user::current_user,
};

use super::support::response_from_result;

#[utoipa::path(
    get,
    path = "/api/v1/auth/me",
    tag = "auth",
    responses((status = 200, description = "Current user", body = ApiEnvelope<MeResponse>))
)]
pub async fn me(
    Extension(auth_context): Extension<AuthContext>,
    State(state): State<Arc<ServerState>>,
) -> ApiResponseResult<MeResponse> {
    response_from_result(current_user(state, auth_context.claims).await)
}
