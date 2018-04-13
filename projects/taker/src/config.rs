
#[derive(Serialize)]
struct Config {
    files: Vec<String>,
}

impl Config {
    fn new() -> Config {
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            files: [
                "~/.bash_profile",
                "~/.ssh/",
                "~/.emacs.el",
                "~/.aws",
                "~/.config/awesome/rc.lua",
            ],
        }
    }
}

pub fn parse(path: String) -> Config {
    let mut config_toml = String::new();

    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(_) => {
            error!("Could not find config file, using default!");
            return Config::new();
        }
    };

    file.read_to_string(&mut config_toml)
        .unwrap_or_else(|err| panic!("Error while reading config: [{}]", err));

    let mut parser = Parser::new(&config_toml);
    let toml = parser.parse();

    if toml.is_none() {
        for err in &parser.errors {
            let (loline, locol) = parser.to_linecol(err.lo);
            let (hiline, hicol) = parser.to_linecol(err.hi);
            println!(
                "{}:{}:{}-{}:{} error: {}",
                path, loline, locol, hiline, hicol, err.desc
            );
        }
        panic!("Unrecoverable Error");
    }

    let config = Config(toml.unwrap());
    match toml::decode(config) {
        Some(t) => t,
        None => panic!("Error while deserializing config"),
    }
}
