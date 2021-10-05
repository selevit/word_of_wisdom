use std::env;
use std::error::Error;
use word_of_wisdom::{server_addr_from_env, setup_logging, Server};

fn main() -> Result<(), Box<dyn Error>> {
    setup_logging();
    let addr = server_addr_from_env();
    let responses_file =
        env::var("RESPONSES_FILENAME").unwrap_or_else(|_| "./server_responses.txt".into());
    let mut server = Server::new_from_file(&responses_file)?;
    if let Ok(c) = env::var("PUZZLE_COMPLEXITY") {
        let err_msg = "PUZZLE_COMPLEXITY must be an integer from 1 to 10";
        let complexity = c.parse::<u8>().expect(err_msg);
        if !(1..=10).contains(&complexity) {
            return Err(err_msg.into());
        }
        server.set_puzzle_complexity(complexity);
    }
    server.run(addr.as_str())
}
