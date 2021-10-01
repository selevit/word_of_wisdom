pub mod proof_of_work;
use proof_of_work::proto::{Puzzle, PuzzleSolution, SolutionState, SOLUTION_SIZE};
use proof_of_work::PuzzleSolver;

use anyhow::Result;
use proof_of_work::Transport;
use rand::seq::SliceRandom;
use std::error::Error;
use std::fs;
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::Arc;
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
}

struct Server {
    responses: Vec<String>,
}

impl Server {
    pub fn new_from_file(filename: &str) -> Result<Self, Box<dyn Error>> {
        let mut responses = Vec::<String>::new();
        for val in fs::read_to_string(filename)?.split("\n\n") {
            let mut s = String::from(val);
            s.truncate(val.trim_end_matches(&['\r', '\n'][..]).len());
            responses.push(s);
        }
        if responses.is_empty() {
            return Err(format!("file is empty: {}", filename).into());
        }
        println!(
            "Loaded {} response phrases from {}",
            responses.len(),
            filename
        );
        Ok(Server { responses })
    }

    pub fn run(self) -> Result<(), Box<dyn Error + 'static>> {
        Arc::new(self).run_listener()?;
        Ok(())
    }

    fn handle_connection(&self, conn: &mut Connection) -> Result<()> {
        let mut client = Transport::new(conn.stream.try_clone()?);
        let puzzle = Puzzle::default();
        let solver = PuzzleSolver::new(&puzzle);

        loop {
            match conn.state {
                ClientState::Initial => {
                    client.send(&puzzle)?;
                    println!("Puzzle sent");
                    conn.state = ClientState::PuzzleSent;
                }
                ClientState::PuzzleSent => {
                    println!("Waiting for solution");
                    let solution = client.receive::<PuzzleSolution>(SOLUTION_SIZE)?;
                    println!("Solution received");

                    if solver.is_valid_solution(&solution) {
                        println!("Solution accepted");
                        client.send(&SolutionState::ACCEPTED)?;
                        client.send_with_varsize(self.random_response())?;
                    } else {
                        println!("Solution rejected");
                    }

                    conn.stream.shutdown(Shutdown::Both)?;
                    println!("Connection closed");
                    break;
                }
            }
        }

        Ok(())
    }

    fn random_response(&self) -> &String {
        self.responses.choose(&mut rand::thread_rng()).unwrap()
    }

    fn run_listener(self: Arc<Self>) -> Result<(), Box<dyn Error + 'static>> {
        let listener = TcpListener::bind("0.0.0.0:4444")?;
        println!("Listening on port 4444");

        for stream in listener.incoming() {
            let server_clone = self.clone();
            match stream {
                Ok(stream) => {
                    println!("New TCP connection: {}", stream.peer_addr()?);
                    thread::spawn(move || {
                        let mut conn = Connection::new(stream);
                        if let Err(e) = server_clone.handle_connection(&mut conn) {
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
}

const RESPONSES_FILENAME: &str = "server_responses.txt";

fn main() -> Result<(), Box<dyn Error>> {
    let server = Server::new_from_file(RESPONSES_FILENAME)?;
    server.run()
}
