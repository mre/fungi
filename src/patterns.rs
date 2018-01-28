// https://doc.rust-lang.org/stable/book/second-edition/ch18-00-patterns.html
// https://doc.rust-lang.org/stable/book/second-edition/ch18-01-all-the-places-for-patterns.html

// All the places Patterns may be used
// - Match Arms
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
}
