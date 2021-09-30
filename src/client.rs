pub mod proof_of_work;
use bincode::{deserialize, serialize};
use proof_of_work::proto::{
    Puzzle, PuzzleSolution, QuoteSize, SolutionResponse, WordOfWisdomQuote,
};
use proof_of_work::verify_challenge;
use rand::Rng;
use std::io::prelude::*;
use std::io::Write;
use std::mem::size_of;
use std::net::TcpStream;

fn solve_challenge(puzzle: &Puzzle) -> PuzzleSolution {
    loop {
        let solution = rand::thread_rng().gen::<[u8; 16]>();
        let solution = PuzzleSolution::new(solution);
        if verify_challenge(puzzle, &solution) {
            return solution;
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:4444")?;

    // receiving puzzle
    let mut buf = [0u8; size_of::<Puzzle>()];
    stream.read_exact(&mut buf)?;
    let puzzle: Puzzle = deserialize(&buf).unwrap();

    println!("puzzle received (complexity: {})", puzzle.complexity);
    println!("solving...");

    // solving puzzle
    let solution = solve_challenge(&puzzle);
    println!("puzzle solved");

    // sending solution
    let serialized_solution = serialize(&solution).unwrap();
    stream.write_all(&serialized_solution[..])?;

    // receiving solution result
    let mut buf = [0u8; 4];
    stream.read_exact(&mut buf)?;
    let solution_response: SolutionResponse = deserialize(&buf).unwrap();

    match solution_response {
        SolutionResponse::REJECTED => {
            println!("solution rejected");
        }
        SolutionResponse::ACCEPTED => {
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
