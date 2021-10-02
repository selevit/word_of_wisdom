mod wow_tcp_server;
use env_logger::Env;
use std::env;
pub use wow_tcp_server::*;

pub fn setup_logging() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
}

pub fn server_addr_from_env() -> String {
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".into());
    let port = env::var("PORT").unwrap_or_else(|_| "4444".into());
    format!("{}:{}", host, port)
}
