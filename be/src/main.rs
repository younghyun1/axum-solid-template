#![allow(clippy::module_inception, clippy::question_mark)]

use std::sync::Arc;

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
pub mod router;
pub mod util;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let server_state: ServerState = match init::server_init::init_server_state() {
        Ok(server_state) => server_state,
        Err(error) => {
            eprintln!("Failed to initialize server state");
            eprintln!("{error:#}");
            std::process::exit(1);
        }
    };

    let shared_state = Arc::new(server_state);

    match init::server_init::run_server(shared_state).await {
        Ok(()) => {}
        Err(error) => {
            error!(error = %error, "Server failed");
            std::process::exit(1);
        }
    }
}
