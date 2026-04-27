use crate::init::config::server_config::ServerConfig;

#[derive(Debug, Clone)]
pub struct ServerState {
    pub server_config: ServerConfig,
}

impl ServerState {
    pub fn new(server_config: ServerConfig) -> Self {
        Self { server_config }
    }
}
