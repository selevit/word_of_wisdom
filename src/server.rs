pub mod proof_of_work;
use env_logger::Env;
use proof_of_work::Server;
use std::error::Error;
use std::env;

fn main() -> Result<(), Box<dyn Error>> {
    let responses_file = env::var("RESPONSES_FILE").unwrap_or_else(|_| "./server_responses.txt".into());
    let addr = env::var("LISTEN_ADDR").unwrap_or_else(|_| "127.0.0.1:4444".into());
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let server = Server::new_from_file(&responses_file)?;
    server.run(&addr)
}
