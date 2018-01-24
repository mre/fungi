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
//
// https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap_or_else
// fn unwrap_or_else<F>(self, op: F) -> T
// where F: FnOnce(E) -> T,
//   Unwraps a result, yielding the content of an Ok. If the value is an Err
//   then it calls op with its value.

fn main() {
    // https://doc.rust-lang.org/std/env/fn.args.html
    // pub fn args() -> Args
    // https://doc.rust-lang.org/std/env/struct.Args.html
    // pub struct Args { /* fields omitted */ }
    // An iterator over the arguments of a process, yielding a String value for
    // each argument.
    //
    // The env::args function returns an iterator. Rather than collecting the
    // iterator values into a vector and then passing a slice to Config::new,
    // now we're passing ownership of the iterator returned from env::args to
    // Config::new directly.
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprint!("Application error: {}\n", e);

        process::exit(1);
    }
}
