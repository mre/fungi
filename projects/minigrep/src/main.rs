use std::env;
use std::process;
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

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprint!("Problem parsing arguments: {}\n\n", err);
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let mut f = File::open(config.filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    println!("With text:\n{}", contents);
}
