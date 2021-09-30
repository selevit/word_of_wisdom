pub mod proof_of_work;
use anyhow::Result;
use proof_of_work::proto::{Puzzle, SolutionState, SOLUTION_STATE_SIZE};
use proof_of_work::Transport;
use std::mem::size_of;
use std::net::TcpStream;

fn main() -> Result<()> {
    let mut server = Transport::new(TcpStream::connect("127.0.0.1:4444")?);

    let puzzle = server.receive::<Puzzle>(size_of::<Puzzle>())?;
    println!("Puzzle received (complexity: {})", puzzle.complexity);

    println!("Solving...");
    let result = puzzle.solve();
    println!("Puzzle solved with {} attempts", result.hashes_tried);

    server.send(&result.solution)?;
    match server.receive::<SolutionState>(SOLUTION_STATE_SIZE)? {
        SolutionState::ACCEPTED => {
            println!("Solution accepted");
            let server_msg_size = server.receive::<usize>(size_of::<usize>())?;
            let server_msg = server.receive::<String>(server_msg_size)?;
            println!("\n> \n> {} \n> ", server_msg);
        }
        SolutionState::REJECTED => {
            eprintln!("Solution rejected");
        }
    }

    Ok(())
}
