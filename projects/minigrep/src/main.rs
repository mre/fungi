use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    // https://doc.rust-lang.org/std/macro.eprint.html
    // https://doc.rust-lang.org/std/process/fn.exit.html
    if args.len() < 3 {
        eprint!("Not enough parameters!\n\n");
        process::exit(1);
    }
    // let program_name = &args[0]
    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);
}
