use tracing::{info, warn};

use crate::init::state::server_state::ServerState;

pub async fn clear_public_cache(state: &ServerState, reason: &str) {
    match state.marketplace_public_cache.clear().await {
        Ok(()) => {
            info!(reason, "Cleared marketplace public cache");
        }
        Err(error) => {
            warn!(
                reason,
                error = %error,
                "Failed to clear marketplace public cache"
            );
        }
    }
}
