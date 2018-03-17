// https://hyper.rs/guides/
//
// [dependencies]
// futures = "0.1.14"
// hyper = "0.11"

// https://github.com/klausi/rustnish/blob/gh-pages/_posts/2018-03-11-crashing-a-rust-hyper-server-with-a-denial-of-service-attack.md
// https://klausi.github.io/rustnish/2018/03/11/crashing-a-rust-hyper-server-with-a-denial-of-service-attack.html
// https://hyper.rs/guides/server/hello-world/
//

extern crate hyper;
// use hyper::header::{ContentType, ContentLength};
use hyper::header::*;
use hyper::server::*;
// use hyper::server::{Http, Request, Response, Service};

static PHRASE: &'static [u8] = b"Hello World!";

fn main() {
    let addr = ([127, 0, 0, 1], 3000).into();
    loop {
        let new_service = const_service(service_fn(|_| {
            Ok(Response::<hyper::Body>::new()
                .with_header(ContentLength(PHRASE.len() as u64))
                .with_header(ContentType::plaintext())
                .with_body(PHRASE))
        }));

        let server = Http::new().bind(&addr, new_service).unwrap();

        // Official fix:
        // https://docs.rs/hyper/0.11.22/hyper/server/struct.Http.html#method.sleep_on_errors
        //
        // let server = Http::new()
        //   .sleep_on_errors(true)
        //   .bind(&addr, new_service)
        //   .unwrap();
        println!(
            "Listening on http://{} with 1 thread.",
            server.local_addr().unwrap()
        );
        if let Err(e) = server.run() {
            println!("Error: {:?}", e);
        }
    }
}
