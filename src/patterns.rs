// https://doc.rust-lang.org/stable/book/second-edition/ch18-00-patterns.html
// https://doc.rust-lang.org/stable/book/second-edition/ch18-01-all-the-places-for-patterns.html
// https://doc.rust-lang.org/stable/book/second-edition/ch18-02-refutability.html
// https://doc.rust-lang.org/stable/book/second-edition/ch18-03-pattern-syntax.html

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

    // Matching literals
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // Matching Named Variables
    // Because match starts a new scope, variables declared as part of a pattern
    // inside the match expression will shadow those with the same name outside
    // the match construct---as is the case with all variables.
    // A match statement with an arm that introduces a shadowed variable y.
    {
        let x = Some(5);
        let y = 10;
        match x {
            Some(50) => println!("Got 50"),
            Some(y) => println!("Matched, y = {:?}", y),
            _ => println!("Default case, x = {:?}", x),
        }
        println!("at the end: x = {:?}, y = {:?}", x, y);
    }

    // Multiple Patterns
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // Matching Ranges of values with ...
    let x = 5;

    match x {
        1...5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'...'j' => println!("early ASCII letter"),
        'k'...'z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // Destructuring to Break Apart Values
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    // Destructuring Enum variants
    #[allow(dead_code)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);

    #[allow(non_shorthand_field_patterns)]
    match msg {
        Message::Quit => println!("The Quit variant has no data to destructure."),
        Message::Move { x: x, y: y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
    }

    // Destructuring references
    // When the value we're matching to our pattern contains a reference, we
    // need to destructure the reference from the value, which we can do can by
    // specifying a & in the pattern.
    let points = vec![
        Point { x: 0, y: 0 },
        Point { x: 1, y: 5 },
        Point { x: 10, y: -3 },
    ];

    let _sum_of_squares: i32 = points.iter().map(|&Point { x, y }| x * x + y * y).sum();
    // If we had not included the & in &Point { x, y } we'd get a type mismatch
    // error, because iter would then iterate over references to the items in
    // the vector rather than the values themselves. The error would look like
    // this:
    // error[E0308]: mismatched types
    //     -->
    //     |
    // 14 |         .map(|Point { x, y }| x * x + y * y)
    //     |               ^^^^^^^^^^^^ expected &Point, found struct `Point`
    //     |
    // = note: expected type `&Point`
    //     found type `Point`
    //     This tells us that Rust was expecting our closure to match &Point,
    //     but we tried to match directly to a Point value, and not a reference
    //     to a Point.

    // Destructuring Structs and Tuples
    #[allow(unused_variables)]
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    // Ignore patterns
    fn foo(_: i32, y: i32) {
        println!("This code only uses the y parameter: {}", y);
    }

    foo(3, 4);

    // Using an underscore within patterns that match Some variants when we
    // don't need to use the value inside the Some.
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => println!("Some numbers: {}, {}, {}", first, third, fifth),
    }

    // Difference between _ and _x
    let s = Some(String::from("Hello!"));
    if let Some(_s) = s {
        println!("found a string");
    }

    // move occurs because the value has type `std::string::String`, which does not implement the `Copy` trait
    // println!("{:?}", s);
    // We receive an error because the s value will still be moved into _s,
    // which prevents us from using s again. Using the underscore by itself,
    // however, doesn't ever bind to the value.

    let s = Some(String::from("Hello!"));
    if let Some(_) = s {
        println!("found a string");
    }
    println!("{:?}", s);

    // Ignoring Remaining Parts of a Value with ..
    // The .. pattern will ignore any parts of a value that we haven't
    // explicitly matched in the rest of the pattern.

    struct PointXYZ {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = PointXYZ { x: 0, y: 0, z: 0 };
    match origin {
        PointXYZ { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }

    // But the usage of .. must be unambiguous:
    // match numbers {
    //     (.., second, ..) => {
    //         println!("Some numbers: {}", second)
    //     },
    // }
    // error: `..` can only be used once per tuple or tuple struct pattern
    //   --> src/main.rs:5:22
    //   |
    // 5 |         (.., second, ..) => {
    //   |

    // ref and ref mut to create references in patterns
    // Here we'll look at using ref to make references so ownership of the
    // values isn't moved to variables in the pattern.

    // let robot_name = Some(String::from("Bors"));
    // match robot_name {
    //     Some(name) => println!("Found a name: {}", name),
    //     None => (),
    // }
    // println!("robot_name is: {:?}", robot_name);
    // This example will fail because the value inside Some in robot_name is
    // moved to within the match when name binds to that value.
    // In order to fix this code, we want to have the Some(name) pattern borrow
    // that part of robot_name rather than taking ownership.
    // Outside of patterns, we've seen that the way to borrow a value is to
    // create a reference using &, so you may think the solution is changing
    // Some(name) to Some(&name).
    // However, we saw "Destructuring to Break Apart Values" section that
    // & in patterns does not create a reference, it matches an existing
    // reference in the value. Because & already has that meaning in patterns,
    // we can't use & to create a reference in a pattern.
    // Instead, to create a reference in a pattern, we do this by using the ref
    // keyword before the new variable.

    let robot_name = Some(String::from("Bors"));

    match robot_name {
        // Creating a reference so that a pattern variable does not take
        // ownership of a value
        Some(ref name) => println!("Found a name: {}", name),
        None => (),
    }

    println!("robot_name is: {:?}", robot_name);

    // This example will compile because the value in the Some variant in
    // robot_name is not moved into the match; the match only took a reference
    // to the data in robot_name rather than moving it.
    // To create a mutable reference in order to be able to mutate a value
    // matched in a pattern, use ref mut instead of &mut for the same reason
    // that we use ref instead of &: &mut in patterns is for matching existing
    // mutable references, not creating new ones.

    let mut robot_name = Some(String::from("Bors"));
    match robot_name {
        Some(ref mut name) => *name = String::from("Another name"),
        None => (),
    }
    println!("robot_name is: {:?}", robot_name);
    // Because name is a mutable reference, we need to dereference within the
    // match arm code using the * operator in order to be able to mutate the
    // value.

    // A match guard is an additional if condition specified after the pattern
    // in a match arm that also must match if the pattern matches in order for
    // that arm to be chosen.

    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }
}
