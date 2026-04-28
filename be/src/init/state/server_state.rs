use crate::init::logging::logging::LoggerGuard;
use crate::init::server_config::server_config::ServerConfig;
use crate::init::state::cache::reference_data::ReferenceDataCache;
use crate::init::{db_pool::DbPool, db_pool::DbPoolInitError};
use crate::util::email::sender::MailSender;

#[derive(Debug)]
pub struct ServerState {
    pub server_config: ServerConfig,
    pub logger_guard: LoggerGuard,
    pub db_pool: DbPool,
    pub mail_sender: MailSender,
    pub reference_data_cache: ReferenceDataCache,
}

impl ServerState {
    pub fn new(
        server_config: ServerConfig,
        logger_guard: LoggerGuard,
        db_pool: DbPool,
        mail_sender: MailSender,
        reference_data_cache: ReferenceDataCache,
    ) -> Self {
        Self {
            server_config,
            logger_guard,
            db_pool,
            mail_sender,
            reference_data_cache,
        }
    }

    pub async fn get_conn(
        &self,
    ) -> Result<crate::init::db_pool::DbConnection<'_>, DbPoolInitError> {
        crate::init::db_pool::get_conn(&self.db_pool).await
    }
}
