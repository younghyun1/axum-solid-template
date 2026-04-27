use std::{env, str::FromStr};

use crate::init::config::db_config::{DatabaseConnectionType, DatabaseType};

use super::server_config::{DEPLOYMENT_ENVIRONMENT_KEY, DeploymentEnvironment, ServerConfigError};

pub(super) fn parse_deployment_environment(
    value: String,
) -> Result<DeploymentEnvironment, ServerConfigError> {
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

pub(super) fn parse_database_type(value: String) -> Result<DatabaseType, ServerConfigError> {
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

pub(super) fn parse_database_connection_type(
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

pub(super) fn required_env(env_key: &'static str) -> Result<String, ServerConfigError> {
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

pub(super) fn optional_env(env_key: &'static str) -> Result<Option<String>, ServerConfigError> {
    match env::var(env_key) {
        Ok(value) => Ok(Some(value)),
        Err(env::VarError::NotPresent) => Ok(None),
        Err(env::VarError::NotUnicode(_)) => {
            Err(ServerConfigError::EnvironmentVariableNotUnicode { env_key })
        }
    }
}

pub(super) fn required_bool_env(env_key: &'static str) -> Result<bool, ServerConfigError> {
    let value = match required_env(env_key) {
        Ok(value) => value,
        Err(error) => return Err(error),
    };

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

pub(super) fn required_int_env<T>(env_key: &'static str) -> Result<T, ServerConfigError>
where
    T: FromStr,
    T::Err: ToString,
{
    let value = match required_env(env_key) {
        Ok(value) => value,
        Err(error) => return Err(error),
    };

    match value.parse::<T>() {
        Ok(parsed) => Ok(parsed),
        Err(error) => Err(ServerConfigError::InvalidIntegerEnvironmentVariable {
            env_key,
            value,
            error: error.to_string(),
        }),
    }
}

pub(super) fn normalized_env_value(value: &str) -> String {
    value.trim().to_ascii_lowercase()
}
