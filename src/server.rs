pub mod proof_of_work;
use proof_of_work::proto::{Puzzle, PuzzleSolution, SolutionState, SOLUTION_SIZE};

use anyhow::Result;
use bincode::{deserialize, serialize};
use std::io::prelude::*;
use std::io::Write;
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let puzzle = Puzzle::default();
    let mut puzzle_sent = false;

    loop {
        if !puzzle_sent {
            stream.write_all(&serialize(&puzzle)?)?;
            println!("Puzzle sent");
            puzzle_sent = true;
        } else {
            println!("Waiting for solution");
            let mut buf = [0u8; SOLUTION_SIZE];
            stream.read_exact(&mut buf)?;
            let solution: PuzzleSolution = deserialize(&buf)?;
            println!("Solution received");

            if puzzle.is_valid_solution(&solution) {
                println!("Solution accepted");
                let quote = serialize(&"This is my best quote (Albert Einstein)")?;
                stream.write_all(&serialize(&SolutionState::ACCEPTED)?)?;
                stream.write_all(&serialize(&quote.len())?)?;
                stream.write_all(&quote)?;
            } else {
                println!("Solution rejected");
            }

            stream.shutdown(Shutdown::Both)?;

            println!("Connection closed");

            break;
        }
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:4444")?;
    println!("Listening on port 4444");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New TCP connection: {}", stream.peer_addr()?);

                thread::spawn(move || {
                    if let Err(e) = handle_connection(stream) {
                        eprintln!("Connection error: {}", e);
                    }
                });
            }
            Err(e) => {
                eprintln!("Error establishing TCP connection: {}", e);
            }
        }
    }

    Ok(())
}
