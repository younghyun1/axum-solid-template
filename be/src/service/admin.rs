use std::sync::Arc;

use tracing::error;

use crate::{
    dto::auth::response::DatabaseResetResponse,
    error::prelude::*,
    init::{db_pool::reset_db_migrations, state::server_state::ServerState},
};

pub async fn reset_database(state: Arc<ServerState>) -> ApiResult<DatabaseResetResponse> {
    let reset_summary = match reset_db_migrations(&state.server_config.db_config).await {
        Ok(reset_summary) => reset_summary,
        Err(error) => {
            error!(error = %error, "Failed to reset database migrations");
            return Err(ApiError::from_source(CodeError::DB_UPDATE_ERROR, error));
        }
    };

    match state
        .email_verification_challenge_cache
        .refresh_questionnaire(&state.db_pool)
        .await
    {
        Ok(_) => {}
        Err(error) => {
            error!(error = %error, "Failed to refresh email verification cache after database reset");
            return Err(ApiError::from_source(CodeError::DB_QUERY_ERROR, error));
        }
    }
    state
        .email_verification_challenge_cache
        .clear_runtime_state()
        .await;

    Ok(DatabaseResetResponse {
        reverted_migration_count: reset_summary.reverted_migration_count,
        applied_migration_count: reset_summary.applied_migration_count,
    })
}
