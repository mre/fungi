use std::env;
use std::error::Error;
// https://doc.rust-lang.org/std/macro.eprint.html
// https://doc.rust-lang.org/std/process/fn.exit.html
use std::fs::File;
// https://doc.rust-lang.org/std/io/prelude/index.html
// the std::io module has its own prelude of common things you'll need when
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
// implements the Error trait, but we don't have to specify what particular type
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
    // this is the idiomatic way to indicate that we're calling run for its side
    // effects only; it doesn't return a value we need.
    Ok(())
}

// In this case, we're indicating that the returned vector should contain string
// slices that reference slices of the argument contents (rather than the
// argument query).
// In other words, we're telling Rust that the data returned by the search
// function will live as long as the data passed into the search function in the
// contents argument. This is important! The data referenced by a slice needs to
// be valid in order for the reference to be valid; if the compiler assumed we
// were making string slices of query rather than contents, it would do its
// safety checking incorrectly.
//
// We can write this code in a much more concise way using iterator adaptor
// methods. This also lets us avoid having a mutable intermediate results
// vector. The functional programming style prefers to minimize the amount of
// mutable state to make code clearer. Removing the mutable state might make it
// easier for us to make a future enhancement to make searching happen in
// parallel, since we wouldn't have to manage concurrent access to the results
// vector.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
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

// iterators, while a high-level abstraction, get compiled down to roughly the
// same code as if you'd written the lower-level code yourself. Iterators are
// one of Rust's zero-cost abstractions, by which we mean using the abstraction
// imposes no additional runtime overhead, in the same way that Bjarne
// Stroustrup, the original designer and implementor of C++, defines
// zero-overhead:
//
// In general, C++ implementations obey the zero-overhead principle: What you
// don't use, you don't pay for. And further: What you do use, you couldn't hand
// code any better.
//
//     Bjarne Stroustrup "Foundations of C++"

// an example of iterators usage, that is translated in assembly code where
// there's no loop at all corresponding to the iteration over the values in
// coefficients: Rust knows that there are twelve iterations, so it “unrolls”
// the loop. Unrolling is an optimization that removes the overhead of the loop
// controlling code and instead generates repetitive code for each iteration of
// the loop.

// let buffer: &mut [i32];
// let coefficients: [i64; 12];
// let qlp_shift: i16;
//
// for i in 12..buffer.len() {
//     let prediction = coefficients.iter()
//         .zip(&buffer[i - 12..i])
//         .map(|(&c, &s)| c * s as i64)
//         .sum::<i64>() >> qlp_shift;
//     let delta = buffer[i];
//     buffer[i] = prediction as i32 + delta;
// }
