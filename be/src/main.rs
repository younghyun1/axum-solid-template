#![allow(clippy::module_inception)]

pub mod build_info;
pub mod init;

fn main() {
    match init::server_init::init_server_state() {
        Ok(_server_state) => {}
        Err(error) => {
            eprintln!("Failed to initialize server state: {error:?}");
            std::process::exit(1);
        }
    }
}
