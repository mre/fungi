// https://doc.rust-lang.org/book/second-edition/ch13-01-closures.html

use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure(intensity));
        }
    }
}

// Closure type inference and annotation.
// Closures don’t require you to annotate the types of the parameters or the
// return value like fn functions do.
// Type annotations are required on functions because they are part of an
// explicit interface exposed to your users.
// Closures are usually short and only relevant within a narrow context where
// the compiler is reliably able to infer the types of the parameters and return
// type.
//
// The first time we call example_closure with the String value, the compiler
// infers the type of x and the return type of the closure to be String. Those
// types are then locked in to the closure in example_closure, and we get a type
// error if we try to use a different type with the same closure.
//
// memoization or lazy evaluation.
// We can create a struct that will hold the closure and the resulting value of
// calling the closure. The struct will only execute the closure if we need the
// resulting value, and it will cache the resulting value so that the rest of
// our code doesn’t have to be responsible for saving and reusing the result.
// In order to make a struct that holds a closure, we need to be able to specify
// the type of the closure, because a struct definition needs to know the types
// of each of its fields. Each closure instance has its own unique anonymous
// type: that is, even if two closures have the same signature, their types are
// still considered different.
//
// The Fn traits.
// The Fn traits are provided by the standard library. All closures implement
// one of the traits Fn, FnMut, or FnOnce.
// Adding types: our closure has a parameter of type u32 and returns an u32, so
// the trait bound we specify is Fn(u32) -> u32.

// the definition of the Cacher struct that holds a closure and an optional
// result value:
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

pub fn sample() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}
