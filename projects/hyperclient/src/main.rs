// https://github.com/hyperium/hyper

// [dependencies]
// futures = "0.1"
// hyper = "0.11"
// tokio-core = "0.1"

extern crate futures;
extern crate hyper;
extern crate tokio_core;

use std::io::{self, Write};
use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;

fn main() {
    let mut core = Core::new()?;
    let client = Client::new(&core.handle());
    // We have to create a Core, which is a Tokio event loop, to drive
    // our asynchronous request to completion. With a Core, we can then
    // create a hyper Client that will be registered to our event loop.
    let uri = "http://httpbin.org/ip".parse()?;
    //  let work = client.get(uri);
    // Calling client.get returns a Future that will eventually be
    // fulfilled with a Response.
    // let work = client.get(uri).map(|res| {
    //     println!("Response: {}", res.status());
    // });
    // We chain on the success of that Future using map, and print out
    // the StatusCode of the response. If it isn’t on fire, the server
    // should have responded with a 200 OK status.
    let work = client.get(uri).and_then(|res| {
        println!("Response: {}", res.status());

        res.body().for_each(|chunk| {
            io::stdout()
                .write_all(&chunk)
                .map(|_| ())
                .map_err(From::from)
        })
    });
    // futures are lazy, so the future in work won’t actually do
    // anything until poked, repeatedly. We can tell our event loop (the
    // Core) to “run” the future in work until it succeeds or fails.
    core.run(work)?;
}
