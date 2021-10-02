use std::env;
use std::error::Error;
use word_of_wisdom::{server_addr_from_env, setup_logging, Server};

// TODO: handle sigterm / sigint

fn main() -> Result<(), Box<dyn Error>> {
    setup_logging();
    let addr = server_addr_from_env();
    let responses_file =
        env::var("RESPONSES_FILENAME").unwrap_or_else(|_| "./server_responses.txt".into());
    let server = Server::new_from_file(&responses_file)?;
    server.run(addr.as_str())
}
