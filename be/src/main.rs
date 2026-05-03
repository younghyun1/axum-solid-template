#![allow(clippy::module_inception, clippy::question_mark)]

use std::{sync::Arc, time::Instant};

use crate::init::state::server_state::ServerState;
use mimalloc::MiMalloc;
use tracing::error;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

pub mod build_info;
pub mod controller;
pub mod docs;
pub mod domain;
pub mod dto;
pub mod error;
pub mod init;
pub mod job;
pub mod middleware;
pub mod repository;
pub mod router;
pub mod schema;
pub mod service;
pub mod util;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let startup_started_at = Instant::now();
    let server_state: ServerState = match init::server_init::lifecycle::init_server_state().await {
        Ok(server_state) => server_state,
        Err(error) => {
            eprintln!("Failed to initialize server state");
            eprintln!("{error:#}");
            std::process::exit(1);
        }
    };

    let shared_state = Arc::new(server_state);

    let server_task = tokio::spawn(async move {
        init::server_init::runtime::run_server(shared_state, startup_started_at).await
    });

    match server_task.await {
        Ok(server_result) => match server_result {
            Ok(()) => {}
            Err(error) => {
                error!(error = %error, "Server failed");
                std::process::exit(1);
            }
        },
        Err(error) => {
            error!(error = %error, "Server task failed");
            std::process::exit(1);
        }
    }
}
