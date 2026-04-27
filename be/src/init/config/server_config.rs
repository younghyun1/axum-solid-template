use std::{env, str::FromStr, time::Duration};

use crate::init::config::{
    chatbot::{
        chatbot_config::{ChatbotConfig, ChatbotProvider, RagStorageProvider},
        claude::ClaudeConfig,
        mistral::MistralConfig,
    },
    db_config::{DatabaseConfig, DatabaseConnectionType, DatabaseType},
    file_store_config::FileStoreConfig,
    jwt_config::jwt_config::{JwtConfig, JwtConfigError, JwtIssuer, JwtSecretKey},
};

pub const DEPLOYMENT_ENVIRONMENT_KEY: &str = "DEPLOYMENT_ENVIRONMENT";

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub deployment_environment: DeploymentEnvironment,
    pub https_enabled: bool,
    pub db_config: DatabaseConfig,
    pub file_store_config: FileStoreConfig,
    pub chatbot_config: ChatbotConfig,
    pub jwt_config: JwtConfig,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DeploymentEnvironment {
    Local = 0,
    Development = 1,
    Production = 2,
    ProductionDockerized = 3,
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
    EnvironmentVariableNotUnicode {
        env_key: &'static str,
    },
    JwtConfig(JwtConfigError),
}

impl ServerConfig {
    pub fn from_env() -> Result<Self, ServerConfigError> {
        let deployment_environment =
            parse_deployment_environment(required_env(DEPLOYMENT_ENVIRONMENT_KEY)?)?;
        let https_enabled = required_bool_env("HTTPS_ENABLED")?;
        let db_config = database_config_from_env()?;
        let file_store_config = file_store_config_from_env()?;
        let chatbot_config = chatbot_config_from_env()?;
        let jwt_config = jwt_config_from_env()?;

        Ok(Self {
            deployment_environment,
            https_enabled,
            db_config,
            file_store_config,
            chatbot_config,
            jwt_config,
        })
    }
}

impl From<JwtConfigError> for ServerConfigError {
    fn from(error: JwtConfigError) -> Self {
        Self::JwtConfig(error)
    }
}

fn database_config_from_env() -> Result<DatabaseConfig, ServerConfigError> {
    Ok(DatabaseConfig::new(
        parse_database_type(required_env("DATABASE_TYPE")?)?,
        parse_database_connection_type(required_env("DATABASE_CONNECTION_TYPE")?)?,
        required_env("DATABASE_HOST")?,
        required_int_env("DATABASE_PORT")?,
        required_env("DATABASE_USERNAME")?,
        required_env("DATABASE_PASSWORD")?,
        required_env("DATABASE_NAME")?,
    ))
}

fn file_store_config_from_env() -> Result<FileStoreConfig, ServerConfigError> {
    let file_store_type = required_env("FILE_STORE_TYPE")?;

    match normalized_env_value(&file_store_type).as_str() {
        "local" => Ok(FileStoreConfig::local(required_env(
            "LOCAL_FILE_STORE_BASE_PATH",
        )?)),
        "aws_s3" | "s3" => Ok(FileStoreConfig::aws_s3(
            required_env("AWS_S3_BUCKET_NAME")?,
            required_env("AWS_S3_ACCESS_KEY")?,
            required_env("AWS_S3_SECRET_KEY")?,
            required_env("AWS_S3_REGION")?,
        )),
        _ => Err(ServerConfigError::InvalidEnvironmentVariable {
            env_key: "FILE_STORE_TYPE",
            value: file_store_type,
            expected: "local, aws_s3, or s3",
        }),
    }
}

fn chatbot_config_from_env() -> Result<ChatbotConfig, ServerConfigError> {
    let chatbot_provider = chatbot_provider_from_env()?;
    let chatbot_rag_storage_provider = rag_storage_provider_from_env()?;

    Ok(ChatbotConfig {
        chatbot_provider,
        chatbot_rag_storage_provider,
    })
}

fn chatbot_provider_from_env() -> Result<Option<ChatbotProvider>, ServerConfigError> {
    let provider = optional_env("CHATBOT_PROVIDER")?;

    match provider {
        Some(value) => match normalized_env_value(&value).as_str() {
            "" | "none" | "disabled" => Ok(None),
            "claude" | "anthropic" => Ok(Some(ChatbotProvider::Claude(ClaudeConfig::public_api(
                required_env("ANTHROPIC_API_KEY")?,
                required_env("ANTHROPIC_MODEL")?,
                required_int_env("ANTHROPIC_MAX_TOKENS")?,
            )))),
            "mistral" => Ok(Some(ChatbotProvider::Mistral(MistralConfig::public_api(
                required_env("MISTRAL_API_KEY")?,
                required_env("MISTRAL_MODEL")?,
            )))),
            _ => Err(ServerConfigError::InvalidEnvironmentVariable {
                env_key: "CHATBOT_PROVIDER",
                value,
                expected: "none, disabled, claude, anthropic, or mistral",
            }),
        },
        None => Ok(None),
    }
}

fn rag_storage_provider_from_env() -> Result<Option<RagStorageProvider>, ServerConfigError> {
    let provider = optional_env("CHATBOT_RAG_STORAGE_PROVIDER")?;

    match provider {
        Some(value) => match normalized_env_value(&value).as_str() {
            "" | "none" | "disabled" => Ok(None),
            "in_ram" | "in-memory" | "memory" => Ok(Some(RagStorageProvider::InRam)),
            "pg_vector" | "pgvector" => Ok(Some(RagStorageProvider::PgVector)),
            _ => Err(ServerConfigError::InvalidEnvironmentVariable {
                env_key: "CHATBOT_RAG_STORAGE_PROVIDER",
                value,
                expected: "none, disabled, in_ram, memory, pg_vector, or pgvector",
            }),
        },
        None => Ok(None),
    }
}

fn jwt_config_from_env() -> Result<JwtConfig, ServerConfigError> {
    let secret_key = JwtSecretKey::new(required_env("JWT_SECRET_KEY")?.into_bytes())?;

    let jwt_config = JwtConfig {
        issuer: JwtIssuer(required_env("JWT_ISSUER")?),
        access_token_duration: Duration::from_secs(required_int_env(
            "JWT_ACCESS_TOKEN_DURATION_SECONDS",
        )?),
        refresh_token_duration: Duration::from_secs(required_int_env(
            "JWT_REFRESH_TOKEN_DURATION_SECONDS",
        )?),
        secret_key,
    };

    jwt_config.validate()?;

    Ok(jwt_config)
}

fn parse_deployment_environment(value: String) -> Result<DeploymentEnvironment, ServerConfigError> {
    match normalized_env_value(&value).as_str() {
        "local" => Ok(DeploymentEnvironment::Local),
        "development" | "dev" => Ok(DeploymentEnvironment::Development),
        "production" | "prod" => Ok(DeploymentEnvironment::Production),
        "production_dockerized" | "prod_dockerized" | "dockerized" | "ecs" => {
            Ok(DeploymentEnvironment::ProductionDockerized)
        }
        _ => Err(ServerConfigError::InvalidEnvironmentVariable {
            env_key: DEPLOYMENT_ENVIRONMENT_KEY,
            value,
            expected: "local, development, dev, production, prod, production_dockerized, or ecs",
        }),
    }
}

fn parse_database_type(value: String) -> Result<DatabaseType, ServerConfigError> {
    match normalized_env_value(&value).as_str() {
        "postgres" | "postgresql" => Ok(DatabaseType::Postgres),
        "mysql" => Ok(DatabaseType::MySql),
        "sqlite" => Ok(DatabaseType::Sqlite),
        _ => Err(ServerConfigError::InvalidEnvironmentVariable {
            env_key: "DATABASE_TYPE",
            value,
            expected: "postgres, postgresql, mysql, or sqlite",
        }),
    }
}

fn parse_database_connection_type(
    value: String,
) -> Result<DatabaseConnectionType, ServerConfigError> {
    match normalized_env_value(&value).as_str() {
        "local" => Ok(DatabaseConnectionType::Local),
        "remote" => Ok(DatabaseConnectionType::Remote),
        "domain_socket" | "socket" => Ok(DatabaseConnectionType::DomainSocket),
        _ => Err(ServerConfigError::InvalidEnvironmentVariable {
            env_key: "DATABASE_CONNECTION_TYPE",
            value,
            expected: "local, remote, domain_socket, or socket",
        }),
    }
}

fn required_env(env_key: &'static str) -> Result<String, ServerConfigError> {
    match env::var(env_key) {
        Ok(value) => Ok(value),
        Err(env::VarError::NotPresent) => {
            Err(ServerConfigError::MissingEnvironmentVariable { env_key })
        }
        Err(env::VarError::NotUnicode(_)) => {
            Err(ServerConfigError::EnvironmentVariableNotUnicode { env_key })
        }
    }
}

fn optional_env(env_key: &'static str) -> Result<Option<String>, ServerConfigError> {
    match env::var(env_key) {
        Ok(value) => Ok(Some(value)),
        Err(env::VarError::NotPresent) => Ok(None),
        Err(env::VarError::NotUnicode(_)) => {
            Err(ServerConfigError::EnvironmentVariableNotUnicode { env_key })
        }
    }
}

fn required_bool_env(env_key: &'static str) -> Result<bool, ServerConfigError> {
    let value = required_env(env_key)?;

    match normalized_env_value(&value).as_str() {
        "true" | "1" | "yes" | "y" => Ok(true),
        "false" | "0" | "no" | "n" => Ok(false),
        _ => Err(ServerConfigError::InvalidEnvironmentVariable {
            env_key,
            value,
            expected: "true, false, 1, 0, yes, or no",
        }),
    }
}

fn required_int_env<T>(env_key: &'static str) -> Result<T, ServerConfigError>
where
    T: FromStr,
    T::Err: ToString,
{
    let value = required_env(env_key)?;

    match value.parse::<T>() {
        Ok(parsed) => Ok(parsed),
        Err(error) => Err(ServerConfigError::InvalidIntegerEnvironmentVariable {
            env_key,
            value,
            error: error.to_string(),
        }),
    }
}

fn normalized_env_value(value: &str) -> String {
    value.trim().to_ascii_lowercase()
}
