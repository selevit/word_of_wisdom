use std::net::TcpStream;
use std::io::{Write};
use std::io::BufReader;
use std::io::BufRead;
use std::str;

fn main() -> std::io::Result<()> {
	let mut stream = TcpStream::connect("127.0.0.1:4444")?;
	stream.write_all(b"ping\n")?;
	stream.flush().unwrap();
	let mut reader = BufReader::new(stream.try_clone().unwrap());
	let mut response: Vec<u8> = vec![];
	reader.read_until(b'\n', &mut response)?;

	println!("server responded with: {:?}", str::from_utf8(&response).unwrap());

	Ok(())
}
