use crate::init::logging::logging::LoggerGuard;
use crate::init::server_config::server_config::ServerConfig;
use crate::init::{db_pool::DbPool, db_pool::DbPoolInitError};
use crate::util::email::sender::MailSender;

#[derive(Debug)]
pub struct ServerState {
    pub server_config: ServerConfig,
    pub logger_guard: LoggerGuard,
    pub db_pool: DbPool,
    pub mail_sender: MailSender,
}

impl ServerState {
    pub fn new(
        server_config: ServerConfig,
        logger_guard: LoggerGuard,
        db_pool: DbPool,
        mail_sender: MailSender,
    ) -> Self {
        Self {
            server_config,
            logger_guard,
            db_pool,
            mail_sender,
        }
    }

    pub async fn get_conn(
        &self,
    ) -> Result<crate::init::db_pool::DbConnection<'_>, DbPoolInitError> {
        crate::init::db_pool::get_conn(&self.db_pool).await
    }
}
