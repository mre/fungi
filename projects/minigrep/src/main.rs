extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;

// fn unwrap_or_else<F>(self, op: F) -> T
// where F: FnOnce(E) -> T,
// Unwraps a result, yielding the content of an Ok. If the value is an Err then
// it calls op with its value.
//
// fn count(x: &str) -> usize { x.len() }
// assert_eq!(Ok(2).unwrap_or_else(count), 2);
// assert_eq!(Err("foo").unwrap_or_else(count), 3);

fn main() {
    let args: Vec<String> = env::args().collect();

    // https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap_or_else
    // fn unwrap_or_else<F>(self, op: F) -> T
    // where F: FnOnce(E) -> T,
    //   Unwraps a result, yielding the content of an Ok. If the value is an Err
    //   then it calls op with its value.
    let config = Config::new(&args).unwrap_or_else(|err| {
        // err: &str
        eprint!("Problem parsing arguments: {}\n\n", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
