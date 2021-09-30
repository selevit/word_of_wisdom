pub mod proof_of_work;
use proof_of_work::proto::{Puzzle, PuzzleSolution, SolutionState, SOLUTION_SIZE};
use proof_of_work::PuzzleSolver;

use anyhow::Result;
use proof_of_work::Transport;
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

enum ClientState {
    Initial,
    PuzzleSent,
}

struct Connection {
    stream: TcpStream,
    state: ClientState,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            state: ClientState::Initial,
        }
    }

    pub fn handle(&mut self) -> Result<()> {
        let mut client = Transport::new(self.stream.try_clone()?);
        let puzzle = Puzzle::default();
        let solver = PuzzleSolver::new(&puzzle);

        loop {
            match self.state {
                ClientState::Initial => {
                    client.send(&puzzle)?;
                    println!("Puzzle sent");
                    self.state = ClientState::PuzzleSent;
                }
                ClientState::PuzzleSent => {
                    println!("Waiting for solution");
                    let solution = client.receive::<PuzzleSolution>(SOLUTION_SIZE)?;
                    println!("Solution received");

                    if solver.is_valid_solution(&solution) {
                        println!("Solution accepted");
                        client.send(&SolutionState::ACCEPTED)?;
                        client.send_with_varsize(&String::from(
                            "This is my best quote (Albert Einstein)",
                        ))?;
                    } else {
                        println!("Solution rejected");
                    }

                    self.stream.shutdown(Shutdown::Both)?;
                    println!("Connection closed");
                    break;
                }
            }
        }

        Ok(())
    }
}

fn main() -> Result<()> {
    let listener = TcpListener::bind("0.0.0.0:4444")?;
    println!("Listening on port 4444");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New TCP connection: {}", stream.peer_addr()?);
                thread::spawn(move || {
                    let mut conn = Connection::new(stream);
                    if let Err(e) = conn.handle() {
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
