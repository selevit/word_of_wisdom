pub mod proof_of_work;
use anyhow::Result;
use env_logger::Env;
use proof_of_work::proto::{Puzzle, SolutionState, SOLUTION_STATE_SIZE};
use proof_of_work::{PuzzleSolver, Transport};
use std::error::Error;
use std::mem::size_of;
use std::net::{Shutdown, TcpStream};

struct Client<'a> {
    addr: &'a str,
}

impl<'a> Client<'a> {
    pub fn new(addr: &'a str) -> Self {
        Self { addr }
    }

    pub fn get_response(&self) -> Result<String, Box<dyn Error>> {
        let stream = TcpStream::connect(self.addr)?;
        let mut server = Transport::new(stream.try_clone()?);

        let puzzle: Puzzle = server.receive(size_of::<Puzzle>())?;
        log::info!("Puzzle received (complexity: {})", puzzle.complexity);

        log::info!("Solving...");
        let solver = PuzzleSolver::new(&puzzle); // precomputes a hash to increase the performance
        let result = solver.solve();
        log::info!("Puzzle solved with {} attempts", result.hashes_tried);
        server.send(&result.solution)?;

        let result = match server.receive::<SolutionState>(SOLUTION_STATE_SIZE)? {
            SolutionState::ACCEPTED => {
                log::info!("Solution accepted");
                let server_msg_size: usize = server.receive(size_of::<usize>())?;
                let server_msg: String = server.receive(server_msg_size)?;
                Ok(server_msg)
            }
            SolutionState::REJECTED => Err("Solution rejected".into()),
        };
        let _ = stream.shutdown(Shutdown::Both);
        result
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let client = Client::new("127.0.0.1:4444");
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
