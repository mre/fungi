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
    fn new(args: &[String]) -> Config {
        // https://doc.rust-lang.org/std/macro.eprint.html
        // https://doc.rust-lang.org/std/process/fn.exit.html
        if args.len() < 3 {
            eprint!("Not enough parameters!\n\n");
            process::exit(1);
        }

        // let program_name = args[0].clone();
        // There’s a tendency among many Rustaceans to avoid using clone to fix
        // ownership problems because of its runtime cost.
        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let mut f = File::open(config.filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    println!("With text:\n{}", contents);
}
