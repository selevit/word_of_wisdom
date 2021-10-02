use env_logger::Env;
use std::env;
use std::error::Error;
use word_of_wisdom::Server;

// TODO: handle sigterm / sigint

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".into());
    let port = env::var("PORT").unwrap_or_else(|_| "4444".into());
    let addr = format!("{}:{}", host, port);
    let responses_file =
        env::var("RESPONSES_FILENAME").unwrap_or_else(|_| "./server_responses.txt".into());
    let server = Server::new_from_file(&responses_file)?;
    server.run(addr.as_str())
}
