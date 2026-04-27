use std::{env, fmt};

use tracing::{info, level_filters};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{Layer, layer::SubscriberExt, util::SubscriberInitExt};

use crate::init::server_config::server_config::{DeploymentEnvironment, ServerConfig};

pub const APP_NAME_VERSION_ENV_KEY: &str = "APP_NAME_VERSION";
pub const LOGS_DIR: &str = "./logs/";

pub struct LoggerGuard {
    _file_guard: Option<WorkerGuard>,
}

#[derive(Debug)]
pub enum LoggerInitError {
    MissingEnvironmentVariable { env_key: &'static str },
    EnvironmentVariableNotUnicode { env_key: &'static str },
    SubscriberInit { error: String },
}

impl LoggerGuard {
    fn console_only() -> Self {
        Self { _file_guard: None }
    }

    fn with_file_guard(file_guard: WorkerGuard) -> Self {
        Self {
            _file_guard: Some(file_guard),
        }
    }
}

impl fmt::Debug for LoggerGuard {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LoggerGuard")
            .field("file_logging_enabled", &self._file_guard.is_some())
            .finish()
    }
}

impl fmt::Display for LoggerInitError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoggerInitError::MissingEnvironmentVariable { env_key } => {
                write!(
                    formatter,
                    "missing required environment variable `{env_key}`"
                )
            }
            LoggerInitError::EnvironmentVariableNotUnicode { env_key } => {
                write!(
                    formatter,
                    "environment variable `{env_key}` contains non-unicode data"
                )
            }
            LoggerInitError::SubscriberInit { error } => {
                write!(
                    formatter,
                    "failed to initialize tracing subscriber: {error}"
                )
            }
        }
    }
}

pub fn init_logger(server_config: &ServerConfig) -> Result<LoggerGuard, LoggerInitError> {
    let console_layer = tracing_subscriber::fmt::layer()
        .with_ansi(true)
        .with_target(true)
        .pretty()
        .with_filter(level_filters::LevelFilter::INFO);

    match server_config.deployment_environment {
        DeploymentEnvironment::ProductionDockerized => {
            match tracing_subscriber::registry()
                .with(console_layer)
                .try_init()
            {
                Ok(()) => {
                    info!(
                        deployment_environment = %server_config.deployment_environment,
                        file_logging_enabled = false,
                        "Logger initialized"
                    );
                    Ok(LoggerGuard::console_only())
                }
                Err(error) => Err(LoggerInitError::SubscriberInit {
                    error: error.to_string(),
                }),
            }
        }
        DeploymentEnvironment::Local
        | DeploymentEnvironment::Development
        | DeploymentEnvironment::Production => {
            let app_name_version = match env::var(APP_NAME_VERSION_ENV_KEY) {
                Ok(app_name_version) => app_name_version,
                Err(env::VarError::NotPresent) => {
                    return Err(LoggerInitError::MissingEnvironmentVariable {
                        env_key: APP_NAME_VERSION_ENV_KEY,
                    });
                }
                Err(env::VarError::NotUnicode(_)) => {
                    return Err(LoggerInitError::EnvironmentVariableNotUnicode {
                        env_key: APP_NAME_VERSION_ENV_KEY,
                    });
                }
            };
            let filename = app_name_version.clone();
            let log_directory = format!("{LOGS_DIR}{app_name_version}");
            let file_appender = tracing_appender::rolling::daily(log_directory, filename);
            let (non_blocking_file, file_guard) = tracing_appender::non_blocking(file_appender);

            let file_layer = tracing_subscriber::fmt::layer()
                .with_target(true)
                .json()
                .with_writer(non_blocking_file)
                .with_filter(level_filters::LevelFilter::DEBUG);

            match tracing_subscriber::registry()
                .with(console_layer)
                .with(file_layer)
                .try_init()
            {
                Ok(()) => {
                    info!(
                        deployment_environment = %server_config.deployment_environment,
                        file_logging_enabled = true,
                        "Logger initialized"
                    );
                    Ok(LoggerGuard::with_file_guard(file_guard))
                }
                Err(error) => Err(LoggerInitError::SubscriberInit {
                    error: error.to_string(),
                }),
            }
        }
    }
}
