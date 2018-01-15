// https://doc.rust-lang.org/book/second-edition/ch04-01-what-is-ownership.html

pub fn sample() {
    let s = String::from("hello"); // s comes into scope.

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here.

    let x = 5; // x comes into scope.

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it’s okay to still
                   // use x afterward.
    let _s1 = gives_ownership(); // gives_ownership moves its return
                                 // value into s1.

    let s2 = String::from("hello"); // s2 comes into scope.

    let _s3 = takes_and_gives_back(s2); // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3.
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);

    let s1 = String::from("hello");

    let len = calculate_length_with_ref(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // mutable references have one big restriction: you can only have one
    // mutable reference to a particular piece of data in a particular scope.
    let mut s = String::from("hello");

    change(&mut s);

    // slices
    let s = String::from("hello world");
    let _hello = &s[0..5];
    let _world = &s[6..11];

    let s = String::from("hello");
    let _slice = &s[0..2];
    let _slice = &s[..2];

    let s = String::from("hello");
    let len = s.len();
    let _slice = &s[3..len];
    let _slice = &s[3..];

    let s = String::from("hello");
    let len = s.len();
    let _slice = &s[0..len];
    let _slice = &s[..];
}
// Here, x goes out of scope, then s. But since s's value was moved, nothing
// special happens.
// Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
// moved, so nothing happens. s1 goes out of scope and is dropped.

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

fn calculate_length_with_ref(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// Dangling reference
// fn dangle() -> &String { // dangle returns a reference to a String
//
//     let s = String::from("hello"); // s is a new String
//
//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
// // Danger!

#[allow(dead_code)]
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
