use std::collections::btree_map::BTreeMap;
use std::{env, process};
use std::io::{self, Read, Write};
use std::fmt::Display;
use std::fs::File;

// https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap_or_else
// fn unwrap_or_else<F>(self, op: F) -> T 
// where F: FnOnce(E) -> T, 
// 
//     Unwraps a result, yielding the content of an Ok. If the value is an Err
//     then it calls op with its value.
// 
//     Basic usage:
// 
// fn count(x: &str) -> usize { x.len() }
// 
// assert_eq!(Ok(2).unwrap_or_else(count), 2);
// assert_eq!(Err("foo").unwrap_or_else(count), 3);

fn main() {
    let filename = env::args().nth(1)
        .ok_or("Please supply a file name")
        .unwrap_or_else(|e| exit_err(e, 1));
    
    let mut buf = String::new();
    let mut count = BTreeMap::new();
    
    File::open(&filename)
        .unwrap_or_else(|e| exit_err(e, 2))
        .read_to_string(&mut buf)
        .unwrap_or_else(|e| exit_err(e, 3));
    
    for c in buf.chars() {
        *count.entry(c).or_insert(0) += 1;
    }
    
    println!("Number of occurences per character");
    for (ch, count) in &count {
        println!("{:?}: {}", ch, count);
    }
}

// https://internals.rust-lang.org/t/when-should-i-use-inline/598/4
#[inline]
fn exit_err<T>(msg: T, code: i32) -> ! where T: Display {
    // https://doc.rust-lang.org/std/result/enum.Result.html#method.expect
    // fn expect(self, msg: &str) -> T
    //
    //     Unwraps a result, yielding the content of an Ok.
    //     Panics:
    //     Panics if the value is an Err, with a panic message including the passed message, and the content of the Err.
    //     Basic usage:
    // let x: Result<u32, &str> = Err("emergency failure");
    // x.expect("Testing expect"); // panics with `Testing expect: emergency failure`

    writeln!(&mut io::stderr(), "{}", msg).expect("Could not write to stderr");
    process::exit(code)
}
