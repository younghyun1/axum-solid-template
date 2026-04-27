#![allow(clippy::module_inception, clippy::question_mark)]

use crate::init::state::server_state::ServerState;

pub mod build_info;
pub mod init;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let _server_state: ServerState = match init::server_init::init_server_state() {
        Ok(server_state) => server_state,
        Err(error) => {
            eprintln!("Failed to initialize server state");
            eprintln!("{error:#}");
            std::process::exit(1);
        }
    };
}
