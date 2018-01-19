// https://doc.rust-lang.org/stable/book/second-edition/ch09-02-recoverable-errors-with-result.html

use std::io;
use std::io::Read;
use std::fs::File;

pub fn read_username_from_file_match() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// The ? placed after a Result value is defined to work in almost the same way
// as the match expressions we defined to handle the Result values.
//
// If the value of the Result is an Ok, the value inside the Ok will get
// returned from this expression and the program will continue. If the value is
// an Err, the value inside the Err will be returned from the whole function as
// if we had used the return keyword so the error value gets propagated to the
// calling code.
//
// The one difference between the match expression and what the question mark
// operator does is that when using the question mark operator, error values go
// through the from function defined in the From trait in the standard library.
// Many error types implement the from function to convert an error of one type
// into an error of another type. When used by the question mark operator, the
// call to the from function converts the error type that the question mark
// operator gets into the error type defined in the return type of the current
// function that we’re using ? in. This is useful when parts of a function might
// fail for many different reasons, but the function returns one error type that
// represents all the ways the function might fail. As long as each error type
// implements the from function to define how to convert itself to the returned
// error type, the question mark operator takes care of the conversion
// automatically.
pub fn read_username_from_file_question_mark() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// The ? can only be used in functions that have a return type of Result,
// because it is defined to work in the same way as the match expression we
// defined.
pub fn read_username_from_file_question_mark_chained() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

pub fn not_necessary_to_defend_against_errors() {
    use std::net::IpAddr;

    let _home = "127.0.0.1".parse::<IpAddr>().unwrap();
}

pub fn validation_loop() {
    loop {
        // snip

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess < 1 || guess > 100 {
            println!("The secret number will be between 1 and 100.");
            continue;
        }

        match guess.cmp(&secret_number) {
            // snip
        }
}

fn main() {}
