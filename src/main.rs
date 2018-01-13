extern crate rand;

use std::io;
use std::env;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // a RNG that is local to the current thread of execution and seeded by the
    // operating system
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // https://doc.rust-lang.org/std/env/fn.var.html
    let key = "VERBOSE";
    match env::var(key) {
        Ok(val) => {
            println!("{}: {:?}", key, val);
            println!("The secret number is: {}", secret_number);
        }
        Err(_) => (),
    }

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        // returns a Result, the variants are Ok or Err.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // https://doc.rust-lang.org/std/primitive.str.html#method.parse
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        // variants for Ordering are Less, Greater, and Equal.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
