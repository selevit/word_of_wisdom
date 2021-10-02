mod proto;
use anyhow::Result;
use proto::{
    Puzzle, PuzzleSolution, SolutionState, PUZZLE_SIZE, SOLUTION_SIZE, SOLUTION_STATE_SIZE,
};
use rand::seq::SliceRandom;
use rand::Rng;
use serde::de::DeserializeOwned;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::error::Error;
use std::fs;
use std::io::{Read, Write};
use std::mem::size_of;
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

pub const DEFAULT_COMPLEXITY: u8 = 4;

// A separate solver structure is needed in order not to blow the protocol structures.
pub struct PuzzleSolver<'a> {
    puzzle: &'a Puzzle,
    precomputed_hash: Sha256,
}

pub struct SolvingResult {
    pub solution: PuzzleSolution,
    pub hashes_tried: u128,
}

impl<'a> PuzzleSolver<'a> {
    pub fn new(puzzle: &'a Puzzle) -> Self {
        let mut precomputed_hash = Sha256::new();
        precomputed_hash.update(puzzle.value);
        Self {
            puzzle,
            precomputed_hash,
        }
    }

    pub fn is_valid_solution(&self, solution: &PuzzleSolution) -> bool {
        let mut hasher = self.precomputed_hash.clone();
        hasher.update(solution);

        let hash = hasher.finalize();
        let mut leading_zeros = 0;

        for c in hash.iter().take(self.puzzle.complexity as usize / 2 + 1) {
            if c >> 4 == 0 {
                leading_zeros += 1;
            } else {
                break;
            }
            if c & 0xF == 0 {
                leading_zeros += 1;
            } else {
                break;
            }
        }

        log::debug!("Hash: {:x}, leading zeros: {}", hash, leading_zeros);
        leading_zeros >= self.puzzle.complexity
    }

    pub fn solve(&self) -> SolvingResult {
        let mut rng = rand::thread_rng();
        let mut hashes_tried: u128 = 0;
        loop {
            let solution = rng.gen::<PuzzleSolution>();
            hashes_tried += 1;
            if self.is_valid_solution(&solution) {
                return SolvingResult {
                    solution,
                    hashes_tried,
                };
            }
        }
    }
}

impl Puzzle {
    pub fn new(complexity: u8) -> Self {
        let value = rand::thread_rng().gen::<[u8; PUZZLE_SIZE]>();
        Puzzle { complexity, value }
    }
}

impl Default for Puzzle {
    fn default() -> Self {
        Self::new(DEFAULT_COMPLEXITY)
    }
}

pub struct Transport<T: Read + Write> {
    c: T,
}

impl<T> Transport<T>
where
    T: Read + Write,
{
    pub fn new(c: T) -> Self {
        Self { c }
    }

    pub fn send<V>(&mut self, value: &V) -> Result<()>
    where
        V: Serialize,
    {
        self.c.write_all(&bincode::serialize(value)?)?;
        Ok(())
    }

    pub fn send_with_varsize<V>(&mut self, value: &V) -> Result<()>
    where
        V: Serialize,
    {
        let data = bincode::serialize(value)?;
        let len = bincode::serialize(&data.len())?;
        self.c.write_all(&len)?;
        self.c.write_all(&data)?;
        Ok(())
    }

    pub fn receive<R>(&mut self, size: usize) -> Result<R>
    where
        R: DeserializeOwned,
    {
        let mut buf: Vec<u8> = vec![0; size];
        self.c.read_exact(&mut buf)?;
        let result: R = bincode::deserialize(&buf)?;
        Ok(result)
    }
}

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

pub struct Server {
    responses: Vec<String>,
    puzzle_complexity: u8,
}

impl<'a> Server {
    pub fn new_from_file(filename: &str) -> Result<Self, Box<dyn Error>> {
        let mut responses = Vec::<String>::new();
        log::info!("Loading response phrases from {}", filename);
        for val in fs::read_to_string(filename)?.split("\n\n") {
            responses.push(val.trim_matches(&['\r', '\n', ' '][..]).into());
        }
        log::info!("{} phrases loaded", responses.len(),);
        Self::new_with_responses(responses)
    }

    pub fn new_with_responses(responses: Vec<String>) -> Result<Self, Box<dyn Error>> {
        if responses.is_empty() {
            return Err("responses must not be empty".into());
        }
        Ok(Server {
            responses,
            puzzle_complexity: DEFAULT_COMPLEXITY,
        })
    }

    pub fn set_puzzle_complexity(&mut self, complexity: u8) {
        self.puzzle_complexity = complexity;
    }

    pub fn run(self, addr: &'a str) -> Result<(), Box<dyn Error>> {
        Arc::new(self).run_listener(addr)?;
        Ok(())
    }

    fn run_listener(self: Arc<Self>, addr: &'a str) -> Result<(), Box<dyn Error>> {
        let listener = TcpListener::bind(addr)?;
        log::info!("Listening on {}", addr);

        for stream in listener.incoming() {
            let server_clone = self.clone();
            match stream {
                Ok(stream) => {
                    log::info!("New TCP connection: {}", stream.peer_addr()?);
                    thread::spawn(move || {
                        let mut conn = Connection::new(stream);
                        if let Err(e) = server_clone.handle_connection(&mut conn) {
                            eprintln!("Connection error: {}", e);
                        }
                    });
                }
                Err(e) => {
                    log::error!("Error establishing TCP connection: {}", e);
                }
            }
        }

        Ok(())
    }

    fn handle_connection(&self, conn: &mut Connection) -> Result<()> {
        let mut client = Transport::new(conn.stream.try_clone()?);
        let puzzle = Puzzle::new(self.puzzle_complexity);
        let solver = PuzzleSolver::new(&puzzle);

        loop {
            match conn.state {
                ClientState::Initial => {
                    client.send(&puzzle)?;
                    log::info!("Puzzle sent");
                    conn.state = ClientState::PuzzleSent;
                }
                ClientState::PuzzleSent => {
                    log::info!("Waiting for solution");
                    let solution: PuzzleSolution = client.receive(SOLUTION_SIZE)?;
                    log::info!("Solution received");

                    if solver.is_valid_solution(&solution) {
                        log::info!("Solution accepted");
                        client.send(&SolutionState::ACCEPTED)?;
                        client.send_with_varsize(self.random_response())?;
                    } else {
                        client.send(&SolutionState::REJECTED)?;
                        log::error!("Solution rejected");
                    }

                    conn.stream.shutdown(Shutdown::Both)?;
                    log::info!("Connection closed");
                    break;
                }
            }
        }

        Ok(())
    }

    fn random_response(&self) -> &String {
        self.responses.choose(&mut rand::thread_rng()).unwrap()
    }
}

pub struct Client<'a> {
    addr: &'a str,
}

impl<'a> Client<'a> {
    pub fn new(addr: &'a str) -> Self {
        Self { addr }
    }

    pub fn get_response(&self) -> Result<String, Box<dyn Error>> {
        let stream = TcpStream::connect(self.addr)?;
        let mut server = Transport::new(stream.try_clone()?);

        let puzzle: Puzzle = server.receive(size_of::<Puzzle>())?;
        log::info!("Puzzle received (complexity: {})", puzzle.complexity);

        log::info!("Solving...");
        let solver = PuzzleSolver::new(&puzzle); // precomputes a hash to increase the performance
        let result = solver.solve();
        log::info!("Puzzle solved with {} attempts", result.hashes_tried);
        server.send(&result.solution)?;

        let result = match server.receive::<SolutionState>(SOLUTION_STATE_SIZE)? {
            SolutionState::ACCEPTED => {
                log::info!("Solution accepted");
                let server_msg_size: usize = server.receive(size_of::<usize>())?;
                let server_msg: String = server.receive(server_msg_size)?;
                Ok(server_msg)
            }
            SolutionState::REJECTED => Err("Solution rejected".into()),
        };
        let _ = stream.shutdown(Shutdown::Both);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bincode::{deserialize, serialize};
    use mockstream::SharedMockStream;
    use proto::SolutionState;
    use proto::SOLUTION_SIZE;
    use proto::SOLUTION_STATE_SIZE;
    use std::mem::size_of;
    use tempfile::NamedTempFile;

    #[test]
    fn test_puzzle_new() {
        let p = Puzzle::new(5);
        assert_eq!(p.complexity, 5);
        assert_ne!(p.value, [0u8; PUZZLE_SIZE]);
    }

    #[test]
    fn test_puzzle_default() {
        let p = Puzzle::default();
        assert_eq!(p.complexity, DEFAULT_COMPLEXITY);
        assert_ne!(p.value, [0u8; PUZZLE_SIZE]);
    }

    #[test]
    fn test_is_not_valid_solution() {
        let puzzle = Puzzle::new(30);
        let solver = PuzzleSolver::new(&puzzle);
        assert_eq!(solver.is_valid_solution(&[0u8; SOLUTION_SIZE]), false);
    }

    #[test]
    fn test_puzzle_solve() {
        let puzzle = Puzzle::new(3);
        let solver = PuzzleSolver::new(&puzzle);
        let result = solver.solve();
        assert!(result.hashes_tried > 0);
        assert!(solver.is_valid_solution(&result.solution));
        let mut hasher = Sha256::default();
        hasher.update(puzzle.value);
        hasher.update(result.solution);
        let hash_hex = format!("{:x}", hasher.finalize());
        assert!(hash_hex.starts_with("000"));
    }

    #[test]
    fn test_transport_send() {
        let mut mock_stream = SharedMockStream::new();
        let mut transport = Transport::<SharedMockStream>::new(mock_stream.clone());

        transport.send(&SolutionState::ACCEPTED).unwrap();
        let received = mock_stream.pop_bytes_written();
        assert_eq!(received.len(), SOLUTION_STATE_SIZE);
        assert_eq!(received, serialize(&SolutionState::ACCEPTED).unwrap());

        let puzzle = Puzzle::default();
        transport.send(&puzzle).unwrap();
        let received = mock_stream.pop_bytes_written();
        assert_eq!(received.len(), size_of::<Puzzle>());
        assert_eq!(received, serialize(&puzzle).unwrap());
    }

    #[test]
    fn test_transport_send_with_varsize() {
        let mut mock_stream = SharedMockStream::new();
        let mut transport = Transport::<SharedMockStream>::new(mock_stream.clone());
        let sent_message = String::from("hello, world");

        transport.send_with_varsize(&sent_message).unwrap();
        let received_data = mock_stream.pop_bytes_written();
        let size: usize = deserialize(&received_data[..size_of::<usize>()]).unwrap();
        assert_eq!(size, serialize(&sent_message).unwrap().len());

        let received_message: String = deserialize(&received_data[size_of::<usize>()..]).unwrap();
        assert_eq!(sent_message, received_message);
    }

    #[test]
    fn test_transport_receive() {
        let mut mock_stream = SharedMockStream::new();
        let mut transport = Transport::<SharedMockStream>::new(mock_stream.clone());

        let sent_puzzle = Puzzle::default();
        let bin_data = serialize(&sent_puzzle).unwrap();
        mock_stream.push_bytes_to_read(&bin_data);

        let received_puzzle = transport.receive::<Puzzle>(size_of::<Puzzle>()).unwrap();
        assert_eq!(sent_puzzle, received_puzzle);
    }

    #[test]
    fn test_server_new_from_file() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "response 1 \n\n").unwrap();
        writeln!(file, " \rresponse 2 \n").unwrap();
        writeln!(file, "\nresponse 3").unwrap();
        file.flush().unwrap();

        let server = Server::new_from_file(file.path().to_str().unwrap()).unwrap();
        assert_eq!(
            server.responses,
            vec![
                String::from("response 1"),
                String::from("response 2"),
                String::from("response 3"),
            ]
        );
    }

    #[test]
    fn test_client_and_server() {
        let addr = "127.0.0.1:4000";
        let mut server = Server::new_with_responses(vec![
            String::from("response 1"),
            String::from("response 2"),
        ])
        .unwrap();
        server.set_puzzle_complexity(3);
        thread::spawn(move || {
            server.run(addr).unwrap();
        });
        let client = Client::new(addr);
        let response = client.get_response().unwrap();
        assert!(response == "response 1" || response == "response 2")
    }

    #[test]
    fn test_server_invalid_solution() {
        let addr = "127.0.0.1:4001";
        let mut server = Server::new_with_responses(vec![String::from("response 1")]).unwrap();
        server.set_puzzle_complexity(30);
        thread::spawn(move || {
            server.run(addr).unwrap();
        });
        let mut transport = Transport::new(TcpStream::connect(addr).unwrap());
        transport.receive::<Puzzle>(size_of::<Puzzle>()).unwrap();
        transport.send(&[0u8; SOLUTION_SIZE]).unwrap();
        let result: SolutionState = transport.receive(SOLUTION_STATE_SIZE).unwrap();
        assert_eq!(result, SolutionState::REJECTED);
    }
}
