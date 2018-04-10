// https://github.com/kbknapp/clap-rs/
extern crate clap;
use clap::{Arg, App};

// https://github.com/rust-lang-nursery/log
// https://github.com/sebasmagri/env_logger/
// https://docs.rs/env_logger/*/env_logger/
#[macro_use]
extern crate log;
extern crate env_logger;

extern crate taker;
use log::Level;
use std::io;

fn run_app() -> Result<bool, io::Error> {
    env_logger::init();

    let matches = App::new("taker")
        .version("1.0")
        .author("zeroed")
        .about("Does awesome things")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .get_matches();
    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let config = matches.value_of("config").unwrap_or("default.conf");
    println!("Value for config: {}", config);
    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    match matches.occurrences_of("v") {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        3 | _ => println!("Don't be crazy"),
    };

    if log_enabled!(Level::Debug) {
        info!("running the taker CLI");
    }
    taker::run()
}

fn main() {
    ::std::process::exit(match run_app() {
       Ok(_) => 0,
       Err(err) => {
           eprintln!("error: {:?}", err);
           1
       }
    });
}

