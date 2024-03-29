// When you concatenate strings, you need to allocate memory to store
// the result. The easiest to start with is String and &str:

fn one() {
    let mut owned_string: String = "hello ".to_owned();
    let borrowed_string: &str = "world";

    owned_string.push_str(borrowed_string);
    println!("{}", owned_string);
}

// Here, we have an owned string that we can mutate. This is efficient as
// it potentially allows us to reuse the memory allocation. There's a
// similar case for String and String, as &String can be dereferenced as
// &str.

fn two() {
    let mut owned_string: String = "hello ".to_owned();
    let another_owned_string: String = "world".to_owned();

    owned_string.push_str(&another_owned_string);
    println!("{}", owned_string);
}

// After this, another_owned_string is untouched (note no mut
// qualifier). There's another variant that consumes the String but
// doesn't require it to be mutable. This is an implementation of the
// Add trait that takes a String as the left-hand side and a &str as the
// right-hand side:

fn three() {
    let owned_string: String = "hello ".to_owned();
    let borrowed_string: &str = "world";

    let new_owned_string = owned_string + borrowed_string;
    println!("{}", new_owned_string);
}

// Note that owned_string is no longer accessible after the call to +.
//
//
// What if we wanted to produce a new string, leaving both untouched? The
// simplest way is to use format!:

fn four() {
    let borrowed_string: &str = "hello ";
    let another_borrowed_string: &str = "world";

    let together = format!("{}{}", borrowed_string, another_borrowed_string);
    println!("{}", together);
}

// Note that both input variables are immutable, so we know that they
// aren't touched. If we wanted to do the same thing for any combination
// of String, we can use the fact that String also can be formatted:

fn five() {
    let owned_string: String = "hello ".to_owned();
    let another_owned_string: String = "world".to_owned();

    let together = format!("{}{}", owned_string, another_owned_string);
    println!("{}", together);
}

// You don't have to use format! though. You can clone one string and
// append the other string to the new string:

fn six() {
    let owned_string: String = "hello ".to_owned();
    let borrowed_string: &str = "world";

    let together = owned_string.clone() + borrowed_string;
    println!("{}", together);
}

// Note - all of the type specification I did is redundant - the
// compiler can infer all the types in play here. I added them simply to
// be clear to people new to Rust, as I expect this question to be
// popular with that group!

// - https://doc.rust-lang.org/std/string/struct.String.html#deref-methods
// - https://doc.rust-lang.org/std/string/struct.String.html#impl-Add%3C%26%27a%20str%3E
// - https://doc.rust-lang.org/std/macro.format.html
// - https://doc.rust-lang.org/std/string/struct.String.html#impl-Clone

// Source:
// - https://stackoverflow.com/a/30154791
