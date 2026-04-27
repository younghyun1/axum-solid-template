use std::{env, fmt, path::Path};

use crate::init::{
    config::server_config::server_config::{
        DEPLOYMENT_ENVIRONMENT_KEY, ServerConfig, ServerConfigError,
    },
    state::server_state::ServerState,
};

#[derive(Debug)]
pub enum ServerInitError {
    DeploymentEnvironmentNotUnicode,
    DotenvLoad(dotenvy::Error),
    ServerConfig(ServerConfigError),
}

impl fmt::Display for ServerInitError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServerInitError::DeploymentEnvironmentNotUnicode => {
                write!(
                    formatter,
                    "DEPLOYMENT_ENVIRONMENT contains non-unicode data"
                )
            }
            ServerInitError::DotenvLoad(error) => {
                write!(formatter, "failed to load .env file: {error}")
            }
            ServerInitError::ServerConfig(error) => {
                write!(formatter, "failed to build server config: {error}")
            }
        }
    }
}

pub fn init_server_state() -> Result<ServerState, ServerInitError> {
    match load_dotenv_if_deployment_environment_is_missing() {
        Ok(()) => {}
        Err(error) => {
            return Err(error);
        }
    }

    let server_config: ServerConfig = match ServerConfig::from_env() {
        Ok(server_config) => server_config,
        Err(error) => {
            return Err(ServerInitError::ServerConfig(error));
        }
    };

    Ok(ServerState::new(server_config))
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
