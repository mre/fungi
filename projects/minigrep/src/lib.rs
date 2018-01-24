use std::env;
use std::error::Error;
// https://doc.rust-lang.org/std/macro.eprint.html
// https://doc.rust-lang.org/std/process/fn.exit.html
use std::fs::File;
// https://doc.rust-lang.org/std/io/prelude/index.html
// the std::io module has its own prelude of common things you’ll need when
// working with I/O.
use std::io::prelude::*;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    // The 'static lifetime is the entire duration of the program. All string
    // literals have the 'static lifetime, which we can choose to annotate as
    // follows:
    //   let s: &'static str = "I have a static lifetime.";
    // The text of this string is stored directly in the binary of your program
    // and the binary of your program is always available. Therefore, the
    // lifetime of all string literals is 'static.
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        // we can change the new function to take ownership of an iterator as
        // its argument instead of borrowing a slice. We'll use the iterator
        // functionality instead of the code that checks the length of the slice
        // and indexes into specific locations.
        // Once Config::new takes ownership of the iterator and stops using
        // indexing operations that borrow, we can move the String values from
        // the iterator into Config rather than calling clone and making a new
        // allocation.

        // Skip the position 0: the program name
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
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

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    // This Ok(()) syntax may look a bit strange at first, but using () like
    // this is the idiomatic way to indicate that we’re calling run for its side
    // effects only; it doesn’t return a value we need.
    Ok(())
}

// In this case, we’re indicating that the returned vector should contain string
// slices that reference slices of the argument contents (rather than the
// argument query).
// In other words, we’re telling Rust that the data returned by the search
// function will live as long as the data passed into the search function in the
// contents argument. This is important! The data referenced by a slice needs to
// be valid in order for the reference to be valid; if the compiler assumed we
// were making string slices of query rather than contents, it would do its
// safety checking incorrectly.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
        Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
        Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
