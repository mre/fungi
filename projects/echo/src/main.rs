// A tiny async echo server with Tokio
extern crate futures;
extern crate tokio;
extern crate tokio_io;

use futures::{Future, Stream};
use tokio::executor::current_thread;
use tokio::net::TcpListener;
use tokio_io::{io, AsyncRead};

static LOCAL_BINDING_ADDR: &'static str = "0.0.0.0";
static LOCAL_BINDING_PORT: &'static str = "8080";

// https://doc.rust-lang.org/std/string/struct.String.html#method.with_capacity
// pub fn with_capacity(capacity: usize) -> String
fn concat_strings(a: &str, b: &str) -> String {
    let mut res = String::with_capacity(a.len() + b.len());
    res.push_str(a);
    res.push_str(b);
    res
}

fn concat_all_strings(vargs: &[&str]) -> String {
    let capacity = vargs
        .iter()
        .map(|s: &&str| s.chars().count())
        .fold(0, |acc, len| acc + len);
    let mut res = String::with_capacity(capacity);
    vargs.iter().map
    res.push_str(a);
    res.push_str(b);
    res
}

fn local_binding() -> String {
    unimplemented!()
}

fn main() {
    // Bind the server's socket
    let addr = "127.0.0.1:12345".parse().unwrap();
    let tcp = TcpListener::bind(&addr).unwrap();

    // Iterate incoming connections
    let server = tcp.incoming()
        .for_each(|tcp| {
            // Split up the read and write halves
            let (reader, writer) = tcp.split();

            // Copy the data back to the client
            let conn = io::copy(reader, writer)
            // print what happened
            .map(|(n, _, _)| {
                println!("wrote {} bytes", n)
            })
            // Handle any errors
            .map_err(|err| {
                println!("IO error {:?}", err)
            });

            // Spawn the future as a concurrent task
            current_thread::spawn(conn);

            Ok(())
        })
        .map_err(|err| {
            println!("server error {:?}", err);
        });

    // Spin up the server on the event loop
    current_thread::run(|_| {
        current_thread::spawn(server);
    });
}
