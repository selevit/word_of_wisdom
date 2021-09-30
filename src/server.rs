pub mod proof_of_work;

use proof_of_work::verify_challenge;
use rand::Rng;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

const PUZZLE_COMPLEXITY: u8 = 3;
const CHALLENGE_SIZE: usize = 32;

fn handle_connection(mut stream: TcpStream) {
    let mut challenge_sent = false;
    let challenge = rand::thread_rng().gen::<[u8; CHALLENGE_SIZE]>();

    loop {
        if !challenge_sent {
            stream
                .write_all(format!("{}\n", PUZZLE_COMPLEXITY).as_bytes())
                .unwrap();
            stream.write_all(&challenge[..]).unwrap();
            stream.write_all(b"\n").unwrap();
            println!("challenge sent");
            challenge_sent = true;
        } else {
            let mut reader = BufReader::new(stream.try_clone().unwrap());
            let mut solution: Vec<u8> = vec![];
            println!("waiting for the solution");
            let read_bytes = reader.read_until(b'\n', &mut solution).unwrap();
            let solution = &solution[..solution.len() - 1];
            println!("solution received: {:?}", solution);
            if read_bytes > 0 && verify_challenge(PUZZLE_COMPLEXITY, &challenge[..], solution) {
                println!("solution accepted");
                let _ = stream.write_all(b"This is my best quote (Albert Einstein)\n");
            } else {
                println!("solution rejected");
                let _ = stream.write_all(b"Invalid solution\n");
            }
            println!("connection closed\n");
            let _ = stream.shutdown(Shutdown::Both);
            break;
        }
    }
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:4444").expect("error binding tcp socket");
    println!("Listening on port 4444");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New tcp connection: {}", stream.peer_addr().unwrap());
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
