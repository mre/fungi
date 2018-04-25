// https://github.com/kbknapp/clap-rs/
#[macro_use]
extern crate clap;
use clap::{App, Arg};

// https://github.com/rust-lang-nursery/log
// https://github.com/sebasmagri/env_logger/
// https://docs.rs/env_logger/*/env_logger/
#[macro_use]
extern crate log;
extern crate env_logger;

extern crate taker;

use log::Level;
use std::env;
use std::error::Error;
use std::io;

fn run_app() -> Result<bool, io::Error> {
    env_logger::init();
    // let version = format!("{}.{}.{}{}",
    //                  env!("CARGO_PKG_VERSION_MAJOR"),
    //                  env!("CARGO_PKG_VERSION_MINOR"),
    //                  env!("CARGO_PKG_VERSION_PATCH"),
    //                  option_env!("CARGO_PKG_VERSION_PRE").unwrap_or(""));

    let app = App::new("taker")
        .version(crate_version!())
        .author("zeroed")
        .about("Backup, tag, package and zip a bunch of files")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom [c]onfig file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("verbosity")
                .short("v")
                .long("verbose")
                .takes_value(false)
                .multiple(true)
                .help("Sets the level of [v]erbosity"),
        )
        .arg(
            Arg::with_name("execute")
                .short("x")
                .takes_value(false)
                .multiple(false)
                .help("e[x]ecute the taker"),
        );
    
    let mut a = app.clone();
    let matches = app.get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    match matches.value_of("config") {
        Some(c) => {
            println!("Value for config file: {}", c);
            env::set_var(taker::ENV_CFG, c);
            assert_eq!(env::var(taker::ENV_CFG), Ok(c.to_string()));
        }
        None => println!("using the default config"),
    }

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    match matches.occurrences_of("verbosity") {
        0 => {
            println!("No verbose info (warn)");
            env::set_var(taker::LOG_CFG, "taker=warn");
        }
        1 => {
            println!("Some verbose info (info)");
            env::set_var(taker::LOG_CFG, "taker=info");
        }
        2 => {
            println!("More verbose info (debug)");
            env::set_var(taker::LOG_CFG, "taker=debug");
        }
        3 | _ => println!("Don't be crazy"),
    };

    if log_enabled!(Level::Debug) {
        info!("running the taker CLI");
    }

    if matches.is_present("execute") {
        return taker::run(taker::config());
    } else {
        taker::config();
        a.print_long_help().expect("something is broken");
    }
    Ok(false)
}

fn main() {
    ::std::process::exit(match run_app() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {:?}", err.description());
            1
        }
    });
}
