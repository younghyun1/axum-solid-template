use std::{fmt, net::IpAddr};

use crate::init::{
    chatbot::chatbot_config::ChatbotConfig,
    db_config::DatabaseConfig,
    file_store_config::FileStoreConfig,
    jwt_config::jwt_config::{JwtConfig, JwtConfigError},
};

pub const DEPLOYMENT_ENVIRONMENT_KEY: &str = "DEPLOYMENT_ENVIRONMENT";

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub deployment_environment: DeploymentEnvironment,
    pub https_enabled: bool,
    pub server_bind_ip: IpAddr,
    pub server_port: u16,
    pub http_redirect_port: u16,
    pub db_config: DatabaseConfig,
    pub file_store_config: FileStoreConfig,
    pub chatbot_config: ChatbotConfig,
    pub jwt_config: JwtConfig,
    pub cert_config: Option<CertConfig>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CertConfig {
    pub cert_chain_path: String,
    pub private_key_path: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DeploymentEnvironment {
    Local = 0,
    Development = 1,
    Production = 2,
    ProductionDockerized = 3,
}

impl DeploymentEnvironment {
    pub fn as_str(self) -> &'static str {
        match self {
            DeploymentEnvironment::Local => "local",
            DeploymentEnvironment::Development => "development",
            DeploymentEnvironment::Production => "production",
            DeploymentEnvironment::ProductionDockerized => "production_dockerized",
        }
    }
}

#[derive(Debug)]
pub enum ServerConfigError {
    MissingEnvironmentVariable {
        env_key: &'static str,
    },
    InvalidEnvironmentVariable {
        env_key: &'static str,
        value: String,
        expected: &'static str,
    },
    InvalidIntegerEnvironmentVariable {
        env_key: &'static str,
        value: String,
        error: String,
    },
    InvalidIpAddressEnvironmentVariable {
        env_key: &'static str,
        value: String,
        error: String,
    },
    EnvironmentVariableNotUnicode {
        env_key: &'static str,
    },
    JwtConfig(JwtConfigError),
}

impl From<JwtConfigError> for ServerConfigError {
    fn from(error: JwtConfigError) -> Self {
        Self::JwtConfig(error)
    }
}

impl fmt::Display for DeploymentEnvironment {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl fmt::Display for ServerConfigError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServerConfigError::MissingEnvironmentVariable { env_key } => {
                write!(
                    formatter,
                    "missing required environment variable `{env_key}`"
                )
            }
            ServerConfigError::InvalidEnvironmentVariable {
                env_key,
                value,
                expected,
            } => write!(
                formatter,
                "invalid environment variable `{env_key}` value `{value}`; expected {expected}"
            ),
            ServerConfigError::InvalidIntegerEnvironmentVariable {
                env_key,
                value,
                error,
            } => write!(
                formatter,
                "invalid integer environment variable `{env_key}` value `{value}`: {error}"
            ),
            ServerConfigError::InvalidIpAddressEnvironmentVariable {
                env_key,
                value,
                error,
            } => write!(
                formatter,
                "invalid IP address environment variable `{env_key}` value `{value}`: {error}"
            ),
            ServerConfigError::EnvironmentVariableNotUnicode { env_key } => {
                write!(
                    formatter,
                    "environment variable `{env_key}` contains non-unicode data"
                )
            }
            ServerConfigError::JwtConfig(_) => write!(formatter, "invalid JWT configuration"),
        }
    }
}
