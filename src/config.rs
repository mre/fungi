// http://rosettacode.org/wiki/Read_a_configuration_file#Rust

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::iter::FromIterator;
use std::path::Path;
 
fn main() {
    let path = String::from("file.conf");
    let cfg = config_from_file(path);
    println!("{:?}", cfg);
}
 
fn config_from_file(path: String) -> Config {
    let path = Path::new(&path);
    let file = File::open(path).expect("File not found or cannot be opened");
    let content = BufReader::new(&file);
    let mut cfg = Config::new();
 
    for line in content.lines() {
        let line = line.expect("Could not read the line");
        // Remove whitespaces at the beginning and end
        let line = line.trim();
 
        // Ignore comments and empty lines
        if line.starts_with("#") || line.starts_with(";") || line.is_empty() {
            continue;
        }
 
        // Split line into parameter name and rest tokens
        // fn split_whitespace(&self) -> SplitWhitespace
        // https://doc.rust-lang.org/std/primitive.str.html#method.split_whitespace
        // Split a string slice by whitespace.
        // 
        // The iterator returned will return string slices that are sub-slices
        // of the original string slice, separated by any amount of whitespace.
        // 
        //     'Whitespace' is defined according to the terms of the Unicode Derived Core Property White_Space.
        // 
        //     Examples
        // 
        //     Basic usage:
        // 
        // let mut iter = "A few words".split_whitespace();
        // assert_eq!(Some("A"), iter.next());
        // assert_eq!(Some("few"), iter.next());
        // assert_eq!(Some("words"), iter.next());
        // assert_eq!(None, iter.next());Run
        //
        // Struct std::str::SplitWhitespace1.1.0 [−] [src]
        // pub struct SplitWhitespace<'a> { /* fields omitted */ }
        //     An iterator over the non-whitespace substrings of a string, separated by any amount of whitespace.
        //     This struct is created by the split_whitespace method on str.
        // Trait Implementations
        // impl<'a> Debug for SplitWhitespace<'a>
        // impl<'a> Clone for SplitWhitespace<'a>
        // impl<'a> Iterator for SplitWhitespace<'a>

        let tokens = Vec::from_iter(line.split_whitespace()); 
        println!("tokens: {:?}", tokens);
        let name = tokens.first().unwrap();
        println!("name: {:?}", name);
        let tokens = tokens.get(1..).unwrap();
 
        // Remove the equal signs (that is optional to set a config variable)
        let tokens = tokens.iter().filter(|t| !t.starts_with("="));
        // Remove comment after the parameters
        let tokens = tokens.take_while(|t| !t.starts_with("#") && !t.starts_with(";"));
        let tokens = tokens.map(|t| {
            println!("values: {:?}", t);
            t
        });
 
        // Concat back the parameters into one string to split for separated parameters
        let mut parameters = String::new();
        tokens.for_each(|t| { parameters.push_str(t); parameters.push(' '); });
        // Splits the parameters and trims
        let parameters = parameters.split(',').map(|s| s.trim());
        // Converts them from Vec<&str> into Vec<String>
        let parameters: Vec<String> = parameters.map(|s| s.to_string()).collect();
 
        // Setting the config parameters
        match name.to_lowercase().as_str() {
            "fullname" => cfg.full_name = parameters.get(0).cloned(),
            "favouritefruit" => cfg.favourite_fruit = parameters.get(0).cloned(),
            "needspeeling" => cfg.needs_peeling = true,
            "seedsremoved" => cfg.needs_peeling = true,
            "otherfamily" => cfg.other_family = Some(parameters),
            _ => (),
        }
    }
 
    cfg
}
 
#[derive(Clone, Debug)]
struct Config {
    full_name: Option<String>,
    favourite_fruit: Option<String>,
    needs_peeling: bool,
    seeds_removed: bool,
    other_family: Option<Vec<String>>,
}
 
impl Config {
    fn new() -> Config {
        Config {
            full_name: None,
            favourite_fruit: None,
            needs_peeling: false,
            seeds_removed: false,
            other_family: None,
        }
    }
}
