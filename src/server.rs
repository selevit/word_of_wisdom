pub mod proof_of_work;
use proof_of_work::proto;

use proof_of_work::verify_challenge;
use std::io::prelude::*;
use std::io::Write;
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

const PUZZLE_COMPLEXITY: u8 = 3;

fn handle_connection(mut stream: TcpStream) {
    let mut challenge_sent = false;
    let puzzle = proto::Puzzle::new(PUZZLE_COMPLEXITY);
    let serialized_puzzle = bincode::serialize(&puzzle).unwrap();

    loop {
        match challenge_sent {
            false => {
                stream.write_all(&serialized_puzzle).unwrap();
                println!("challenge sent");
                challenge_sent = true;
            }
            true => {
                let mut buf = [0u8; 16];
                println!("waiting for the solution");
                stream.read_exact(&mut buf).unwrap();
                let solution: proto::PuzzleSolution = bincode::deserialize(&buf).unwrap();
                println!("solution received");

                let solution_response: proto::SolutionResponse;
                if verify_challenge(&puzzle, &solution) {
                    println!("solution accepted");
                    solution_response = proto::SolutionResponse::ACCEPTED;
                } else {
                    println!("solution rejected");
                    solution_response = proto::SolutionResponse::REJECTED;
                }

                let serialized_solution_response = bincode::serialize(&solution_response).unwrap();
                let _ = stream.write_all(&serialized_solution_response);

                if solution_response == proto::SolutionResponse::ACCEPTED {
                    let quote =
                        proto::WordOfWisdomQuote::new("This is my best quote (Albert Einstein)");
                    let serialized_quote = bincode::serialize(&quote).unwrap();
                    let quote_size = proto::QuoteSize::new(serialized_quote.len());
                    let serialized_quote_size = bincode::serialize(&quote_size).unwrap();
                    stream.write_all(&serialized_quote_size).unwrap();
                    stream.write_all(&serialized_quote).unwrap();
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
