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
#[allow(dead_code)]
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
    // vargs.iter().map(|s: &&str| res.push_str(s));
    // map() is conceptually similar to a for loop. However, as map() is lazy,
    // it is best used when you're already working with other iterators. If
    // you're doing some sort of looping for a side effect, it's considered more
    // idiomatic to use for than map().
    // - https://doc.rust-lang.org/std/convert/trait.AsRef.html#tymethod.as_ref
    // - https://doc.rust-lang.org/std/borrow/trait.Borrow.html
    //
    for s in vargs.iter() {
        res.push_str(s);
    }
    res
}

fn local_binding() -> String {
    // Use AsRef when goal is to simply convert into a reference;
    // Use Borrow when goal is related to writing code that is agnostic to the
    // type of borrow and if is reference or value;
    let vargs: &[&str] = &vec![LOCAL_BINDING_ADDR, ":", LOCAL_BINDING_PORT];
    concat_all_strings(vargs)
}

fn main() {
    // Bind the server's socket
    let addr = local_binding().parse().unwrap();
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
