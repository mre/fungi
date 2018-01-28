// https://doc.rust-lang.org/stable/book/second-edition/ch18-00-patterns.html
// https://doc.rust-lang.org/stable/book/second-edition/ch18-01-all-the-places-for-patterns.html
// https://doc.rust-lang.org/stable/book/second-edition/ch18-02-refutability.html

// All the places Patterns may be used
// - Match Arms
//   match VALUE {
//     PATTERN => EXPRESSION,
//     PATTERN => EXPRESSION,
//     PATTERN => EXPRESSION,
//   }
// - Conditional if let Expressions
pub fn sample() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    // while let Conditional Loops
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // for Loops
    let v = vec![1, 2, 3];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
    // let Statements
    // let PATTERN = EXPRESSION;
    let (_x, _y, _z) = (1, 2, 3);
    // If the number of elements in the pattern don't match the number of
    // elements in the tuple, the overall type won't match and we'll get a
    // compiler error.
    // error[E0308]: mismatched types
    //   --> src/main.rs:2:9
    //   |
    // 2 |     let (x, y) = (1, 2, 3);
    //   |         ^^^^^^ expected a tuple with 3 elements, found one with 2 elements
    //
    // Function Paramenters
    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({}, {})", x, y);
    }

    // A function with parameters that destructure a tuple
    {
        let point = (3, 5);
        print_coordinates(&point);
    }

    // Refutability: wken a pattern might fail to match
    // Patterns come in two forms: refutable and irrefutable. Patterns that will
    // match for any possible value passed are said to be irrefutable.
    // let x = 5;
    // Patterns that may fail to match for some possible value are said to be
    // refutable.
    // if let Some(x) = a_value;
    //
    // let statements, function parameters, and for loops can only accept
    // irrefutable patterns, because the program cannot continue do anything
    // meaningful with values that don't match.
    // he if let and while let expressions are restricted to only accept
    // refutable patterns, because by definition they're intended to handle
    // possible failure---the functionality of a conditional is in its ability
    // to perform differently upon success and failure.

    // try to use a refutable pattern where Rust requires an irrefutable pattern
    // Attempting to use a refutable pattern with:
    //   let Some(x) = some_option_value;
    // If some_option_value was a None value, it would fail to match the pattern
    // Some(x), meaning the pattern is refutable.
    // The let statement, however, can only accept an irrefutable patterns.
    // To fix the case where we have a refutable pattern in a place where an
    // irrefutable pattern is needed, we can change the code that uses the
    // pattern: instead of using let, we can use if let. That way, if the
    // pattern doesn't match, the code will just skip the code in the curly
    // brackets:
    let some_option_value: Option<u8> = None;
    if let Some(x) = some_option_value {
        println!("{}", x);
    }

    //  error[E0162]: irrefutable if-let pattern
    //    --> <anon>:2:8
    //    |
    //  2 | if let x = 5 {
    //    |        ^ irrefutable pattern
    // For this reason, match arms must use refutable patterns, except for the
    // last arm that should match any remaining values with an irrefutable
    // pattern.
}
