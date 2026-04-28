use std::{env, fmt, net::SocketAddr, path::Path, sync::Arc};

use axum::Router;
use axum_server::tls_rustls::RustlsConfig;
use tracing::{error, info};

use crate::init::{
    db_pool::{DbPoolInitError, build_db_pool, run_db_migrations},
    logging::logging::{LoggerGuard, LoggerInitError, init_logger},
    server_config::server_config::{DEPLOYMENT_ENVIRONMENT_KEY, ServerConfig, ServerConfigError},
    state::cache::reference_data::{ReferenceDataCache, ReferenceDataCacheError},
    state::server_state::ServerState,
};
use crate::router::{
    app::build_router,
    redirect::{RedirectPorts, build_redirect_router, redirect_socket_addr},
};
use crate::util::email::sender::{MailSender, MailSenderError};

#[derive(Debug)]
pub enum ServerInitError {
    DeploymentEnvironmentNotUnicode,
    DotenvLoad(dotenvy::Error),
    DbPool(DbPoolInitError),
    Logger(LoggerInitError),
    MailSender(MailSenderError),
    ReferenceDataCache(ReferenceDataCacheError),
    ServerConfig(ServerConfigError),
}

#[derive(Debug)]
pub enum ServerRunError {
    RustlsCryptoProvider { error: String },
    MissingTlsConfig,
    TlsConfig { error: String },
    HttpBind { error: String },
    HttpServe { error: String },
    HttpsServe { error: String },
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
            ServerInitError::ServerConfig(error) => {
                write!(formatter, "failed to build server config: {error}")
            }
        }
    }
}

impl fmt::Display for ServerRunError {
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
            ServerRunError::HttpServe { error } => write!(formatter, "HTTP server failed: {error}"),
            ServerRunError::HttpsServe { error } => {
                write!(formatter, "HTTPS server failed: {error}")
            }
        }
    }
}

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
    ))
}

pub async fn run_server(state: Arc<ServerState>) -> Result<(), ServerRunError> {
    let bind_addr = SocketAddr::new(
        state.server_config.server_bind_ip,
        state.server_config.server_port,
    );
    let router = build_router(state.clone());

    info!(
        bind_addr = %bind_addr,
        https_enabled = state.server_config.https_enabled,
        "Starting Axum server"
    );

    match state.server_config.https_enabled {
        true => run_https_server(state, bind_addr, router).await,
        false => run_http_server(bind_addr, router).await,
    }
}

async fn run_http_server(bind_addr: SocketAddr, router: Router) -> Result<(), ServerRunError> {
    let listener = match tokio::net::TcpListener::bind(bind_addr).await {
        Ok(listener) => listener,
        Err(error) => {
            return Err(ServerRunError::HttpBind {
                error: error.to_string(),
            });
        }
    };

    match axum::serve(listener, router).await {
        Ok(()) => Ok(()),
        Err(error) => Err(ServerRunError::HttpServe {
            error: error.to_string(),
        }),
    }
}

async fn run_https_server(
    state: Arc<ServerState>,
    bind_addr: SocketAddr,
    router: Router,
) -> Result<(), ServerRunError> {
    match rustls::crypto::aws_lc_rs::default_provider().install_default() {
        Ok(()) => {}
        Err(error) => {
            return Err(ServerRunError::RustlsCryptoProvider {
                error: format!("{error:?}"),
            });
        }
    }

    let cert_config = match &state.server_config.cert_config {
        Some(cert_config) => cert_config,
        None => return Err(ServerRunError::MissingTlsConfig),
    };

    let tls_config = match RustlsConfig::from_pem_file(
        &cert_config.cert_chain_path,
        &cert_config.private_key_path,
    )
    .await
    {
        Ok(tls_config) => tls_config,
        Err(error) => {
            return Err(ServerRunError::TlsConfig {
                error: error.to_string(),
            });
        }
    };

    spawn_http_redirector(RedirectPorts {
        http: state.server_config.http_redirect_port,
        https: state.server_config.server_port,
    });

    match axum_server::bind_rustls(bind_addr, tls_config)
        .serve(router.into_make_service())
        .await
    {
        Ok(()) => Ok(()),
        Err(error) => Err(ServerRunError::HttpsServe {
            error: error.to_string(),
        }),
    }
}

fn spawn_http_redirector(ports: RedirectPorts) {
    tokio::spawn(async move {
        let addr = redirect_socket_addr(ports.http);
        let router = build_redirect_router(ports);
        let listener = match tokio::net::TcpListener::bind(addr).await {
            Ok(listener) => listener,
            Err(error) => {
                error!(
                    error = %error,
                    bind_addr = %addr,
                    "Failed to bind HTTP redirect listener"
                );
                return;
            }
        };

        info!(
            http_port = ports.http,
            https_port = ports.https,
            "HTTP to HTTPS redirector started"
        );

        match axum::serve(listener, router).await {
            Ok(()) => {}
            Err(error) => {
                error!(error = %error, "HTTP redirector failed");
            }
        }
    });
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
