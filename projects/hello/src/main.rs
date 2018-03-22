use std::fs::File;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

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

fn main() {
    let r = TcpListener::bind("127.0.0.1:8080");
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
    if buffer.starts_with(get) {
        let mut file = File::open("hello.html").unwrap();

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        let mut file = File::open("404.html").unwrap();
        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();

        let response = format!("{}{}", status_line, contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
