use std::{env, path::Path};

use crate::init::{
    config::server_config::{DEPLOYMENT_ENVIRONMENT_KEY, ServerConfig, ServerConfigError},
    state::server_state::ServerState,
};

#[derive(Debug)]
pub enum ServerInitError {
    DeploymentEnvironmentNotUnicode,
    DotenvLoad(dotenvy::Error),
    ServerConfig(ServerConfigError),
}

pub fn init_server_state() -> Result<ServerState, ServerInitError> {
    load_dotenv_if_deployment_environment_is_missing()?;

    match ServerConfig::from_env() {
        Ok(server_config) => Ok(ServerState::new(server_config)),
        Err(error) => Err(ServerInitError::ServerConfig(error)),
    }
}

fn load_dotenv_if_deployment_environment_is_missing() -> Result<(), ServerInitError> {
    match env::var(DEPLOYMENT_ENVIRONMENT_KEY) {
        Ok(_) => Ok(()),
        Err(env::VarError::NotPresent) => {
            let dotenv_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(".env");

            match dotenvy::from_path(dotenv_path) {
                Ok(_) => Ok(()),
                Err(error) => Err(ServerInitError::DotenvLoad(error)),
            }
        }
        Err(env::VarError::NotUnicode(_)) => Err(ServerInitError::DeploymentEnvironmentNotUnicode),
    }
}
