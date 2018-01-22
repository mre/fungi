use std::error::Error;
use std::fs::File;
// https://doc.rust-lang.org/std/io/prelude/index.html
// the std::io module has its own prelude of common things you’ll need when
// working with I/O.
use std::io::prelude::*;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    // The 'static lifetime is the entire duration of the program. All string
    // literals have the 'static lifetime, which we can choose to annotate as
    // follows:
    //   let s: &'static str = "I have a static lifetime.";
    // The text of this string is stored directly in the binary of your program
    // and the binary of your program is always available. Therefore, the
    // lifetime of all string literals is 'static.
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
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

// just know that Box<Error> means the function will return a type that
// implements the Error trait, but we don’t have to specify what particular type
// the return value will be. This gives us flexibility to return error values
// that may be of different types in different error cases.
pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename).expect("file not found");

    let mut contents = String::new();
    // With `?`, rather than panic! on an error, this will return the error
    // value from the current function for the caller to handle.
    f.read_to_string(&mut contents)?;

    // This Ok(()) syntax may look a bit strange at first, but using () like
    // this is the idiomatic way to indicate that we’re calling run for its side
    // effects only; it doesn’t return a value we need.
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
        Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
