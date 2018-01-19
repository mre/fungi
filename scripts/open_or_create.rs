use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        // The condition if error.kind() == ErrorKind::NotFound is called a
        // match guard: it’s an extra condition on a match arm that further
        // refines the arm’s pattern. This condition must be true for that arm’s
        // code to be run; otherwise, the pattern matching will move on to
        // consider the next arm in the match. The ref in the pattern is needed
        // so error is not moved into the guard condition but is merely
        // referenced by it. The reason ref is used to take a reference in a
        // pattern instead of & in short, in the context of a pattern, & matches
        // a reference and gives us its value, but ref matches a value and gives
        // us a reference to it.
        Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
        },
        Err(error) => panic!("There was a problem opening the file: {:?}", error),
    };

    // Alternatives:
    //
    // If the Result value is the Ok variant, unwrap will return the value
    // inside the Ok. If the Result is the Err variant, unwrap will call the
    // panic! macro for us.
    // let f = File::open("hello.txt").unwrap();
    //
    // Another method, expect, which is similar to unwrap, lets us also choose
    // the panic! error message. Using expect instead of unwrap and providing
    // good error messages can convey your intent and make tracking down the
    // source of a panic easier. The syntax of expect looks like this:
    // let f = File::open("hello.txt").expect("Failed to open hello.txt");
}
