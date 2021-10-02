pub mod proof_of_work;
use env_logger::Env;
use proof_of_work::Server;
use std::error::Error;

const RESPONSES_FILENAME: &str = "server_responses.txt";

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let server = Server::new_from_file(RESPONSES_FILENAME)?;
    server.run("127.0.0.1:4444")
}
