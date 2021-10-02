use anyhow::Result;
use std::error::Error;
use word_of_wisdom::{server_addr_from_env, setup_logging, Client};

fn main() -> Result<(), Box<dyn Error>> {
    setup_logging();
    let addr = server_addr_from_env();
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
