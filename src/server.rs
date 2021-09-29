use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};

fn handle_connection(mut stream: TcpStream) {
    let mut buf = [0u8; 512];
    let peer_addr = stream.peer_addr().unwrap();
    loop {
        let should_continue = match stream.read(&mut buf) {
            Ok(_) => {
                let _ = stream.write_all("pong\n".as_bytes());
                true
            },
            Err(_) => {
                eprintln!("TCP connection terminated: {:?}", peer_addr);
                let _ = stream.shutdown(Shutdown::Both);
                false
            }
        };
        if !should_continue {
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
            },
            Err(e) => {
                eprintln!("Error establishing TCP connection: {}", e);
            }
        }
    }
}
