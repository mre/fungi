//! # Building a Multithreaded Web Server
//!
//! - https://doc.rust-lang.org/book/second-edition/ch20-00-final-project-a-web-server.html
//! - https://doc.rust-lang.org/book/second-edition/ch20-01-single-threaded.html
//! - https://doc.rust-lang.org/book/second-edition/ch20-02-slow-requests.html

use std::fs::File;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

// https://github.com/rust-lang-nursery/log
#[macro_use]
extern crate log;

// HTTP request format:
//
// Method Request-URI HTTP-Version CRLF
// headers CRLF
// message-body
//
// HTTP response format:
//
// HTTP-Version Status-Code Reason-Phrase CRLF
// headers CRLF
// message-body

const BASE_URL: &'static str = "127.0.0.1:8080";

fn main() {
    let r = TcpListener::bind(BASE_URL);
    info!("server binded to {}", BASE_URL);
    let _r = match r {
        Ok(listener) => for stream in listener.incoming() {
            let r_stream = stream;
            let _r_tream = match r_stream {
                Ok(stream) => {
                    println!("Connection established!");
                    handle_connection(stream);
                }
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
    // println!("request: {}", String::from_utf8_lossy(&buffer[..]));
    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
