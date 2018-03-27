//! # Building a Multithreaded Web Server
//!
//! - https://doc.rust-lang.org/book/second-edition/ch20-00-final-project-a-web-server.html
//! - https://doc.rust-lang.org/book/second-edition/ch20-01-single-threaded.html
//! - https://doc.rust-lang.org/book/second-edition/ch20-02-slow-requests.html
//! - https://doc.rust-lang.org/book/second-edition/ch20-03-designing-the-interface.html
//! - https://doc.rust-lang.org/book/second-edition/ch20-04-storing-threads.html
//! - https://doc.rust-lang.org/book/second-edition/ch20-05-sending-requests-via-channels.html

extern crate hello;
use hello::ThreadPool;

use std::fs::File;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

// https://github.com/rust-lang-nursery/log
// https://github.com/sebasmagri/env_logger/
// https://docs.rs/env_logger/*/env_logger/
#[macro_use]
extern crate log;
extern crate env_logger;

use log::Level;
use std::thread;
use std::time::Duration;

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

// RUST_LOG=hello cargo run
fn main() {
    let r = TcpListener::bind(BASE_URL);
    let pool = ThreadPool::new(4);
    env_logger::Builder::from_default_env()
        .default_format_timestamp(true)
        .init();

    if log_enabled!(Level::Info) {
        info!("server binded to {}", BASE_URL);
    }

    let _r = match r {
        Ok(listener) => for stream in listener.incoming() {
            let r_stream = stream;
            let _r_tream = match r_stream {
                Ok(stream) => {
                    pool.execute(|| {
                        println!("Connection established!");
                        handle_connection(stream);
                    });
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
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "templates/hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "templates/404.html")
    };

    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(error) => panic!("There was a problem opening the file: {:?}", error),
    };
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
