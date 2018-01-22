use std::env;
use std::process;
use std::error::Error;
use std::fs::File;
// https://doc.rust-lang.org/std/io/prelude/index.html
// the std::io module has its own prelude of common things you’ll need when
// working with I/O.
use std::io::prelude::*;

struct Config {
    query: String,
    filename: String,
}

impl Config {
    // The 'static lifetime is the entire duration of the program. All string
    // literals have the 'static lifetime, which we can choose to annotate as
    // follows:
    //   let s: &'static str = "I have a static lifetime.";
    // The text of this string is stored directly in the binary of your program
    // and the binary of your program is always available. Therefore, the
    // lifetime of all string literals is 'static.
    fn new(args: &[String]) -> Result<Config, &'static str> {
        // https://doc.rust-lang.org/std/macro.eprint.html
        // https://doc.rust-lang.org/std/process/fn.exit.html
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // let program_name = args[0].clone();
        // There’s a tendency among many Rustaceans to avoid using clone to fix
        // ownership problems because of its runtime cost.
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

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
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    run(config);
}

// just know that Box<Error> means the function will return a type that
// implements the Error trait, but we don’t have to specify what particular type
// the return value will be. This gives us flexibility to return error values
// that may be of different types in different error cases.
fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename).expect("file not found");

    let mut contents = String::new();
    // With `?`, rather than panic! on an error, this will return the error
    // value from the current function for the caller to handle.
    f.read_to_string(&mut contents)?;

    println!("With text:\n{}", contents);

    // This Ok(()) syntax may look a bit strange at first, but using () like
    // this is the idiomatic way to indicate that we’re calling run for its side
    // effects only; it doesn’t return a value we need.
    Ok(())
}
