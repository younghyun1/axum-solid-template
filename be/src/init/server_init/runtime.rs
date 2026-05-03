use std::{net::SocketAddr, sync::Arc, time::Instant};

use axum::Router;
use axum_server::{
    Server,
    tls_rustls::{RustlsAcceptor, RustlsConfig},
};
use tracing::{error, info};

use crate::init::{server_init::error::ServerRunError, state::server_state::ServerState};
use crate::router::{
    app::build_router,
    redirect::{RedirectPorts, build_redirect_router, redirect_socket_addr},
};

pub async fn run_server(
    state: Arc<ServerState>,
    startup_started_at: Instant,
) -> Result<(), ServerRunError> {
    let axum_start_started_at = Instant::now();
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
        true => {
            run_https_server(
                state,
                bind_addr,
                router,
                startup_started_at,
                axum_start_started_at,
            )
            .await
        }
        false => {
            run_http_server(bind_addr, router, startup_started_at, axum_start_started_at).await
        }
    }
}

async fn run_http_server(
    bind_addr: SocketAddr,
    router: Router,
    startup_started_at: Instant,
    axum_start_started_at: Instant,
) -> Result<(), ServerRunError> {
    let listener = match tokio::net::TcpListener::bind(bind_addr).await {
        Ok(listener) => listener,
        Err(error) => {
            return Err(ServerRunError::HttpBind {
                error: error.to_string(),
            });
        }
    };

    info!(
        bind_addr = %bind_addr,
        protocol = "http",
        axum_start_elapsed = ?axum_start_started_at.elapsed(),
        total_startup_elapsed = ?startup_started_at.elapsed(),
        "Axum server started"
    );

    match axum::serve(
        listener,
        router.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    {
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
    startup_started_at: Instant,
    axum_start_started_at: Instant,
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

    let listener = match tokio::net::TcpListener::bind(bind_addr).await {
        Ok(listener) => listener,
        Err(error) => {
            return Err(ServerRunError::HttpsBind {
                error: error.to_string(),
            });
        }
    };

    info!(
        bind_addr = %bind_addr,
        protocol = "https",
        axum_start_elapsed = ?axum_start_started_at.elapsed(),
        total_startup_elapsed = ?startup_started_at.elapsed(),
        "Axum server started"
    );

    let server =
        Server::<SocketAddr>::from_listener(listener).acceptor(RustlsAcceptor::new(tls_config));

    match server
        .serve(router.into_make_service_with_connect_info::<SocketAddr>())
        .await
    {
        Ok(()) => Ok(()),
        Err(error) => Err(ServerRunError::HttpsServe {
            error: error.to_string(),
        }),
    }
}

/// Perform the `spawn_http_redirector` operation as implemented by this function.
///
/// # Arguments
/// * `ports` -
/// # Returns
/// No value is returned (`()`).
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
