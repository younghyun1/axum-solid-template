use std::fmt;

use tracing::{info, level_filters};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{Layer, layer::SubscriberExt, util::SubscriberInitExt};

use crate::build_info::{PROJECT_NAME, PROJECT_VERSION};
use crate::init::server_config::server_config::{DeploymentEnvironment, ServerConfig};

pub const LOGS_DIR: &str = "./logs/";

pub struct LoggerGuard {
    _file_guard: Option<WorkerGuard>,
}

#[derive(Debug)]
pub enum LoggerInitError {
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
            let app_name_version = format!("{PROJECT_NAME}-{PROJECT_VERSION}");
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
