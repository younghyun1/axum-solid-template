use crate::init::logging::logging::LoggerGuard;
use crate::init::server_config::server_config::ServerConfig;

#[derive(Debug)]
pub struct ServerState {
    pub server_config: ServerConfig,
    pub logger_guard: LoggerGuard,
}

impl ServerState {
    pub fn new(server_config: ServerConfig, logger_guard: LoggerGuard) -> Self {
        Self {
            server_config,
            logger_guard,
        }
    }
}
