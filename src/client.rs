pub mod proof_of_work;
use anyhow::Result;
use env_logger::Env;
use proof_of_work::proto::{Puzzle, SolutionState, SOLUTION_STATE_SIZE};
use proof_of_work::{PuzzleSolver, Transport};
use std::mem::size_of;
use std::net::TcpStream;

fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let mut server = Transport::new(TcpStream::connect("127.0.0.1:4444")?);

    let puzzle: Puzzle = server.receive(size_of::<Puzzle>())?;
    log::info!("Puzzle received (complexity: {})", puzzle.complexity);

    log::info!("Solving...");
    let solver = PuzzleSolver::new(&puzzle); // precomputes a hash to increase the performance
    let result = solver.solve();
    log::info!("Puzzle solved with {} attempts", result.hashes_tried);

    server.send(&result.solution)?;
    match server.receive::<SolutionState>(SOLUTION_STATE_SIZE)? {
        SolutionState::ACCEPTED => {
            log::info!("Solution accepted");
            let server_msg_size = server.receive::<usize>(size_of::<usize>())?;
            let server_msg = server.receive::<String>(server_msg_size)?;
            log::info!("Server response: > {}", server_msg);
        }
        SolutionState::REJECTED => {
            log::error!("Solution rejected");
        }
    }

    Ok(())
}
