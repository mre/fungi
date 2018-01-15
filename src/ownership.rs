// https://doc.rust-lang.org/book/second-edition/ch04-01-what-is-ownership.html

pub fn sample() {
    let s = String::from("hello"); // s comes into scope.

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here.

    let x = 5; // x comes into scope.

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it’s okay to still
                   // use x afterward.
} // Here, x goes out of scope, then s. But since s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope.
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope.
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it.

    let some_string = String::from("hello"); // some_string comes into scope.

    some_string // some_string is returned and
                // moves out to the calling
                // function.
}

// takes_and_gives_back will take a String and return one.
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope.
    a_string // a_string is returned and moves out to the calling function.
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String.

    (s, length)
}

