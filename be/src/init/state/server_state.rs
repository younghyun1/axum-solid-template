use crate::init::logging::logging::LoggerGuard;
use crate::init::server_config::server_config::ServerConfig;
use crate::init::state::cache::email_verification::EmailVerificationChallengeCache;
use crate::init::state::cache::reference_data::types::ReferenceDataCache;
use crate::init::state::search::marketplace::index::MarketplaceSearchIndex;
use crate::init::{db_pool::DbPool, db_pool::DbPoolInitError};
use crate::util::email::sender::MailSender;

#[derive(Debug)]
pub struct ServerState {
    pub server_config: ServerConfig,
    pub logger_guard: LoggerGuard,
    pub db_pool: DbPool,
    pub mail_sender: MailSender,
    pub reference_data_cache: ReferenceDataCache,
    pub email_verification_challenge_cache: EmailVerificationChallengeCache,
    pub marketplace_search_index: MarketplaceSearchIndex,
}

impl ServerState {
    pub fn new(
        server_config: ServerConfig,
        logger_guard: LoggerGuard,
        db_pool: DbPool,
        mail_sender: MailSender,
        reference_data_cache: ReferenceDataCache,
        email_verification_challenge_cache: EmailVerificationChallengeCache,
        marketplace_search_index: MarketplaceSearchIndex,
    ) -> Self {
        Self {
            server_config,
            logger_guard,
            db_pool,
            mail_sender,
            reference_data_cache,
            email_verification_challenge_cache,
            marketplace_search_index,
        }
    }

    pub async fn get_conn(
        &self,
    ) -> Result<crate::init::db_pool::DbConnection<'_>, DbPoolInitError> {
        crate::init::db_pool::get_conn(&self.db_pool).await
    }
}
