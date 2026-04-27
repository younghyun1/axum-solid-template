use crate::init::config::{
    chatbot_config::ChatbotConfig, db_config::DatabaseConfig, file_store_config::FileStoreConfig,
};

pub struct ServerConfig {
    pub deployment_environment: DeploymentEnvironment,
    pub https_enabled: bool,
    pub db_config: DatabaseConfig,
    pub file_store_config: FileStoreConfig,
    pub chatbot_config: ChatbotConfig,
}

#[repr(u8)]
enum DeploymentEnvironment {
    Local = 0,
    Development = 1,
    Production = 2,
}
