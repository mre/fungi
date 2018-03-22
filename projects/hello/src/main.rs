use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let r = TcpListener::bind("127.0.0.1:8080");
    let _r = match r {
        Ok(listener) => for stream in listener.incoming() {
            let r_stream = stream;
            let _r_tream = match r_stream {
                Ok(stream) => {
                    println!("Connection established!");
                    handle_connection(stream);
                },
                Err(error) => panic!("a stream was just refused: {:?}", error),
            };
        },
        Err(error) => panic!("There was a problem opening the bind: {:?}", error),
    };
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    // The String::from_utf8_lossy function takes a &[u8] and produces a String.
    println!("request: {}", String::from_utf8_lossy(&buffer[..]));
}
