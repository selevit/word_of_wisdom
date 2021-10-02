pub mod proof_of_work;
use anyhow::Result;
use env_logger::Env;
use proof_of_work::Client;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".into());
    let port = env::var("PORT").unwrap_or_else(|_| "4444".into());
    let addr = format!("{}:{}", host, port);
    let client = Client::new(addr.as_str());
    match client.get_response() {
        Ok(r) => {
            log::info!("Server response: > {}", r);
            Ok(())
        }
        Err(e) => {
            log::error!("{}", e);
            Err(e)
        }
    }
}
