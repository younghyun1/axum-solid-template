use std::env;

use crate::init::{
    db_pool::{build_db_pool, run_db_migrations},
    logging::logging::{LoggerGuard, init_logger},
    server_config::server_config::{DEPLOYMENT_ENVIRONMENT_KEY, ServerConfig},
    server_init::error::ServerInitError,
    state::cache::{
        email_verification::EmailVerificationChallengeCache,
        reference_data::types::ReferenceDataCache,
    },
    state::server_state::ServerState,
};
use crate::util::email::sender::MailSender;

/// Perform the `init_server_state` operation as implemented by this function.
///
/// # Returns
/// A `Result`, either containing the function output or an error.
pub async fn init_server_state() -> Result<ServerState, ServerInitError> {
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

    let logger_guard: LoggerGuard = match init_logger(&server_config) {
        Ok(logger_guard) => logger_guard,
        Err(error) => {
            return Err(ServerInitError::Logger(error));
        }
    };

    match run_db_migrations(&server_config.db_config).await {
        Ok(()) => {}
        Err(error) => {
            return Err(ServerInitError::DbPool(error));
        }
    }

    let db_pool = match build_db_pool(&server_config.db_config).await {
        Ok(db_pool) => db_pool,
        Err(error) => {
            return Err(ServerInitError::DbPool(error));
        }
    };

    let reference_data_cache = match ReferenceDataCache::load(&db_pool).await {
        Ok(reference_data_cache) => reference_data_cache,
        Err(error) => {
            return Err(ServerInitError::ReferenceDataCache(error));
        }
    };
    let email_verification_challenge_cache =
        match EmailVerificationChallengeCache::load(&db_pool).await {
            Ok(email_verification_challenge_cache) => email_verification_challenge_cache,
            Err(error) => {
                return Err(ServerInitError::EmailVerificationChallengeCache(error));
            }
        };

    let mail_sender = match MailSender::from_config(&server_config.mail_config) {
        Ok(mail_sender) => mail_sender,
        Err(error) => {
            return Err(ServerInitError::MailSender(error));
        }
    };

    Ok(ServerState::new(
        server_config,
        logger_guard,
        db_pool,
        mail_sender,
        reference_data_cache,
        email_verification_challenge_cache,
    ))
}

/// Perform the `load_dotenv_if_deployment_environment_is_missing` operation as implemented by this function.
///
/// # Arguments
/// * `) -> Result<(` -
/// # Returns
/// A `Result`, either containing the function output or an error.
fn load_dotenv_if_deployment_environment_is_missing() -> Result<(), ServerInitError> {
    match env::var(DEPLOYMENT_ENVIRONMENT_KEY) {
        Ok(_) => Ok(()),
        Err(env::VarError::NotPresent) => match dotenvy::dotenv() {
            Ok(_) => Ok(()),
            Err(error) => Err(ServerInitError::DotenvLoad(error)),
        },
        Err(env::VarError::NotUnicode(_)) => Err(ServerInitError::DeploymentEnvironmentNotUnicode),
    }
}
