pub mod proof_of_work;
use proof_of_work::proto::{Puzzle, PuzzleSolution, QuoteSize, SolutionState, WordOfWisdomQuote};

use bincode::{deserialize, serialize};
use std::io::prelude::*;
use std::io::Write;
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

const PUZZLE_COMPLEXITY: u8 = 3;

fn handle_connection(mut stream: TcpStream) {
    let mut challenge_sent = false;
    let puzzle = Puzzle::new(PUZZLE_COMPLEXITY);
    let serialized_puzzle = serialize(&puzzle).unwrap();

    loop {
        match challenge_sent {
            false => {
                stream.write_all(&serialized_puzzle).unwrap();
                println!("challenge sent");
                challenge_sent = true;
            }
            true => {
                println!("waiting for the solution");
                let mut buf = [0u8; 16];
                stream.read_exact(&mut buf).unwrap();
                let solution: PuzzleSolution = deserialize(&buf).unwrap();
                println!("solution received");

                let result = puzzle.verify(&solution);
                let _ = stream.write_all(&serialize(&result).unwrap());

                match result {
                    SolutionState::ACCEPTED => {
                        let quote = serialize(&WordOfWisdomQuote::new(
                            "This is my best quote (Albert Einstein)",
                        ))
                        .unwrap();
                        // TODO: try to serialize usize as is
                        stream
                            .write_all(&serialize(&QuoteSize::new(quote.len())).unwrap())
                            .unwrap();
                        stream.write_all(&quote).unwrap();
                    }
                    SolutionState::REJECTED => {
                        println!("solution rejected");
                    }
                }

                let _ = stream.shutdown(Shutdown::Both);
                println!("connection closed");
                break;
            }
        }
    }
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:4444").expect("error binding tcp socket");
    println!("Listening on port 4444");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New TCP connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    handle_connection(stream);
                });
            }
            Err(e) => {
                eprintln!("Error establishing TCP connection: {}", e);
            }
        }
    }
}
