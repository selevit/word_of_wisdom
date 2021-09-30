pub mod proof_of_work;
use proof_of_work::proto::{Puzzle, PuzzleSolution, SolutionState, SOLUTION_SIZE};

use anyhow::Result;
use proof_of_work::Transport;
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

fn handle_connection(stream: TcpStream) -> Result<()> {
    let mut client = Transport::new(stream.try_clone()?);
    let puzzle = Puzzle::default();
    let mut puzzle_sent = false;

    loop {
        if !puzzle_sent {
            client.send(&puzzle)?;
            println!("Puzzle sent");
            puzzle_sent = true;
        } else {
            println!("Waiting for solution");
            let solution = client.receive::<PuzzleSolution>(SOLUTION_SIZE)?;
            println!("Solution received");

            if puzzle.is_valid_solution(&solution) {
                println!("Solution accepted");
                client.send(&SolutionState::ACCEPTED)?;
                client
                    .send_with_varsize(&String::from("This is my best quote (Albert Einstein)"))?;
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
