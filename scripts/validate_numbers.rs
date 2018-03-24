#[macro_use]
extern crate error_chain;
extern crate num;

mod errors {
    error_chain!{
        foreign_links {
            Int(::std::num::ParseIntError);
            Real(::std::num::ParseFloatError);
        }
    }
}
use errors::*;
use std::cmp;
use std::fmt;
use std::str;

quick_main!(run);

fn run() -> Result<()> {
    let a = valid_num("123", 0, 255, "a");
    println!("a={}", a.unwrap());
    match valid_num("245", -20, 20, "b") {
        Ok(b) => println!("b={}", b),
        Err(err) => match Error::from(err) {
            Error(ErrorKind::Int(err), _) => println!("b={}", err),
            err => println!("err: {}", err),
        },
    }
    let c = valid_num("11.54", -20.0, 20.0, "c");
    println!("c={}", c.unwrap());
    let d = valid_num("5.x", -20.0, 20.0, "d");
    println!("d={}", d.unwrap_err());
    Ok(())
}

fn valid_num<T>(s: &str, minimum: T, maximum: T, what: &str) -> Result<T>
where
    T: num::Num + cmp::PartialOrd + Copy + str::FromStr + fmt::Display,
    errors::Error: From<T::Err>,
    T::Err: std::error::Error + Send + 'static,
{
    let n: T = s.parse().chain_err(|| format!("{}:{}", s, what))?; // How to I add: format!(" for {}", what)
    if minimum <= n && n <= maximum {
        return Ok(n);
    }
    bail!("{} must be in range [{}, {}]", what, minimum, maximum)
}
