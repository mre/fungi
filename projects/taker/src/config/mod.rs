extern crate toml;

// https://doc.rust-lang.org/std/io/trait.Read.html
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub files: Vec<String>,
}

macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

impl Default for Config {
    fn default() -> Config {
        Config {
            files: vec_of_strings![
                "~/.bash_profile",
                "~/.ssh/",
                "~/.emacs.el",
                "~/.aws",
                "~/.config/awesome/rc.lua",
                "~/.this/pwsafe.psafe3"
            ],
        }
    }
}

pub fn parse(path: String) -> Config {
    let mut config_toml = String::new();

    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(_) => {
            warn!("Could not find config file {:?}, using default!", &path);
            let cfg: Config = Config::default();
            info!("running with this configuration: {:?}", &cfg);
            return cfg;
        }
    };

    // https://doc.rust-lang.org/std/io/trait.Read.html
    file.read_to_string(&mut config_toml)
        .unwrap_or_else(|err| panic!("Error while reading config: [{}]", err));

    // https://docs.rs/toml/0.4.6/toml/de/fn.from_str.html
    let parsed = toml::from_str(&config_toml);
    match parsed {
        Ok(c) => {
            let cfg: Config = c;
            debug!("running with this configuration: {:?}", cfg);
            return cfg;
        }
        Err(e) => {
            let (line, col) = e.line_col().expect("unparseable parse error");
            error!("{}:{} error: {}", line, col, e.description());
            return Config::default();
        }
    }
}
