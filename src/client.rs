pub mod proof_of_work;
use bincode::{deserialize, serialize};
use proof_of_work::proto::{Puzzle, QuoteSize, SolutionState, WordOfWisdomQuote};
use std::io::prelude::*;
use std::io::Write;
use std::mem::size_of;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:4444")?;

    // receiving puzzle
    let mut buf = [0u8; size_of::<Puzzle>()];
    stream.read_exact(&mut buf)?;
    let puzzle: Puzzle = deserialize(&buf).unwrap();

    println!("puzzle received (complexity: {})", puzzle.complexity);

    // solving puzzle
    println!("solving...");
    let solution = puzzle.solve();
    println!("puzzle solved");

    // sending solution
    stream.write_all(&serialize(&solution).unwrap())?;

    // receiving solution result
    let mut buf = [0u8; 4];
    stream.read_exact(&mut buf)?;
    let solution_response: SolutionState = deserialize(&buf).unwrap();

    match solution_response {
        SolutionState::REJECTED => {
            println!("solution rejected");
        }
        SolutionState::ACCEPTED => {
            println!("solution accepted");

            // receiving response size
            let mut buf = [0u8; size_of::<QuoteSize>()];
            stream.read_exact(&mut buf)?;
            let quote_size: QuoteSize = bincode::deserialize(&buf).unwrap();

            // receving response
            let mut buf: Vec<u8> = vec![0; quote_size.size()];
            stream.read_exact(&mut buf)?;
            let quote: WordOfWisdomQuote = bincode::deserialize(&buf).unwrap();
            println!("\n> \n> {} \n> ", quote.as_str());
        }
    }

    Ok(())
}
