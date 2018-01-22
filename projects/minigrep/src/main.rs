use std::env;
use std::process;
use std::fs::File;
// https://doc.rust-lang.org/std/io/prelude/index.html
// the std::io module has its own prelude of common things you’ll need when
// working with I/O.
use std::io::prelude::*;

fn parse_config(args: &[String]) -> (&str, &str) {
    // https://doc.rust-lang.org/std/macro.eprint.html
    // https://doc.rust-lang.org/std/process/fn.exit.html
    if args.len() < 3 {
        eprint!("Not enough parameters!\n\n");
        process::exit(1);
    }
    // let program_name = &args[0]
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, filename) = parse_config(&args);
    println!("Searching for {}", query);
    println!("In file {}", filename);

    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    println!("With text:\n{}", contents);
}
