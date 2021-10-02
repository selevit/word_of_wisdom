pub mod proof_of_work;
use anyhow::Result;
use env_logger::Env;
use proof_of_work::Client;
use std::error::Error;
use std::env;

fn main() -> Result<(), Box<dyn Error>> {
    let addr = env::var("SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:4444".into());
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let client = Client::new(&addr);
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
