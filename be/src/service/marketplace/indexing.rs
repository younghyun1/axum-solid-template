use tracing::{info, warn};

use crate::init::state::server_state::ServerState;

pub async fn rebuild_search_index(state: &ServerState, reason: &str) {
    match state.marketplace_search_index.rebuild(&state.db_pool).await {
        Ok(indexed_documents) => {
            info!(
                reason,
                indexed_documents, "Rebuilt marketplace search index"
            );
        }
        Err(error) => {
            warn!(
                reason,
                error = %error,
                "Failed to rebuild marketplace search index"
            );
        }
    }
}
