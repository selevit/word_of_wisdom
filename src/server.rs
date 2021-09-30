pub mod proof_of_work;
use proof_of_work::proto::{Puzzle, PuzzleSolution, SolutionState, SOLUTION_SIZE};

use bincode::{deserialize, serialize};
use std::io::prelude::*;
use std::io::Write;
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

fn handle_connection(mut stream: TcpStream) {
    let mut challenge_sent = false;
    let puzzle = Puzzle::default();
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
                let mut buf = [0u8; SOLUTION_SIZE];
                stream.read_exact(&mut buf).unwrap();
                let solution: PuzzleSolution = deserialize(&buf).unwrap();
                println!("solution received");

                if puzzle.is_valid_solution(&solution) {
                    println!("solution accepted");
                    let quote = serialize(&"This is my best quote (Albert Einstein)").unwrap();
                    stream
                        .write_all(&serialize(&SolutionState::ACCEPTED).unwrap())
                        .unwrap();
                    stream.write_all(&serialize(&quote.len()).unwrap()).unwrap();
                    stream.write_all(&quote).unwrap();
                } else {
                    println!("solution rejected");
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
