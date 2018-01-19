use std::io::{BufReader,BufRead};
use std::fs::File;

// https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap
// fn unwrap(self) -> T[src][−]
//     Unwraps a result, yielding the content of an Ok.
//     Panics
//     Panics if the value is an Err, with a panic message provided by the Err's value.
//     Examples
//     Basic usage:
// 
// let x: Result<u32, &str> = Ok(2);
// assert_eq!(x.unwrap(), 2);Run
// let x: Result<u32, &str> = Err("emergency failure");
// x.unwrap(); // panics with `emergency failure`

fn main() {
    let file = File::open("file.txt").unwrap();
    for line in BufReader::new(file).lines() {
        // line :
        // std::result::Result<std::string::String, std::io::Error>
        println!("{}", line.unwrap());
    }
}
