use std::fmt;

use crate::init::{
    db_pool::DbPoolInitError,
    logging::logging::LoggerInitError,
    server_config::server_config::ServerConfigError,
    state::cache::{
        email_verification::EmailVerificationChallengeCacheError,
        reference_data::error::ReferenceDataCacheError,
    },
    state::search::marketplace::error::MarketplaceSearchError,
};
use crate::util::email::sender::MailSenderError;

#[derive(Debug)]
pub enum ServerInitError {
    DeploymentEnvironmentNotUnicode,
    DotenvLoad(dotenvy::Error),
    DbPool(DbPoolInitError),
    Logger(LoggerInitError),
    MailSender(MailSenderError),
    ReferenceDataCache(ReferenceDataCacheError),
    EmailVerificationChallengeCache(EmailVerificationChallengeCacheError),
    MarketplaceSearch(MarketplaceSearchError),
    ServerConfig(ServerConfigError),
}

#[derive(Debug)]
pub enum ServerRunError {
    RustlsCryptoProvider { error: String },
    MissingTlsConfig,
    TlsConfig { error: String },
    HttpBind { error: String },
    HttpsBind { error: String },
    HttpServe { error: String },
    HttpsServe { error: String },
}

impl fmt::Display for ServerInitError {
    /// Perform the `fmt` operation as implemented by this function.
    ///
    /// # Arguments
    /// * `self` -
    /// * `formatter` -
    /// # Returns
    /// Returns the value produced by this function.
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
            ServerInitError::DbPool(error) => {
                write!(formatter, "failed to initialize database pool: {error}")
            }
            ServerInitError::Logger(error) => {
                write!(formatter, "failed to initialize logger: {error}")
            }
            ServerInitError::MailSender(error) => {
                write!(formatter, "failed to initialize mail sender: {error}")
            }
            ServerInitError::ReferenceDataCache(error) => {
                write!(
                    formatter,
                    "failed to initialize reference data cache: {error}"
                )
            }
            ServerInitError::EmailVerificationChallengeCache(error) => {
                write!(
                    formatter,
                    "failed to initialize email verification challenge cache: {error}"
                )
            }
            ServerInitError::MarketplaceSearch(error) => {
                write!(
                    formatter,
                    "failed to initialize marketplace search: {error}"
                )
            }
            ServerInitError::ServerConfig(error) => {
                write!(formatter, "failed to build server config: {error}")
            }
        }
    }
}

impl fmt::Display for ServerRunError {
    /// Perform the `fmt` operation as implemented by this function.
    ///
    /// # Arguments
    /// * `self` -
    /// * `formatter` -
    /// # Returns
    /// Returns the value produced by this function.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServerRunError::RustlsCryptoProvider { error } => {
                write!(
                    formatter,
                    "failed to install rustls crypto provider: {error}"
                )
            }
            ServerRunError::MissingTlsConfig => {
                formatter.write_str("HTTPS is enabled but TLS config is missing")
            }
            ServerRunError::TlsConfig { error } => {
                write!(formatter, "failed to load TLS config: {error}")
            }
            ServerRunError::HttpBind { error } => {
                write!(formatter, "failed to bind HTTP listener: {error}")
            }
            ServerRunError::HttpsBind { error } => {
                write!(formatter, "failed to bind HTTPS listener: {error}")
            }
            ServerRunError::HttpServe { error } => write!(formatter, "HTTP server failed: {error}"),
            ServerRunError::HttpsServe { error } => {
                write!(formatter, "HTTPS server failed: {error}")
            }
        }
    }
}
