pub mod proof_of_work;
use proof_of_work::verify_challenge;
use rand::Rng;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::net::TcpStream;
extern crate hex_slice;
use std::str;

fn solve_challenge(complexity: u8, challenge: &[u8]) -> Vec<u8> {
    loop {
        let solution = rand::thread_rng().gen::<[u8; 32]>();
        if verify_challenge(complexity, challenge, &solution[..]) {
            return solution[..].to_vec();
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:4444")?;
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut complexity: Vec<u8> = vec![];
    println!("getting challenge complexity");
    reader.read_until(b'\n', &mut complexity)?;
    complexity.remove(complexity.len() - 1);
    let complexity = complexity[0] - b'0';

    println!("getting challenge puzzle");
    let mut challenge: Vec<u8> = vec![];
    reader.read_until(b'\n', &mut challenge)?;
    challenge.remove(challenge.len() - 1);

    println!("challenge: {:?}, complexity: {:?}", challenge, complexity);

    println!("solving challenge");
    let solution = solve_challenge(complexity, &challenge[..]);
    println!("challenge solved: {:?}", solution);

    stream.write_all(&solution[..])?;
    stream.write_all(b"\n")?;
    stream.flush()?;

    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut payload: Vec<u8> = vec![];
    reader.read_until(b'\n', &mut payload)?;
    println!(
        "payload received: {}",
        str::from_utf8(&payload[..payload.len() - 1]).unwrap()
    );
    Ok(())
}
