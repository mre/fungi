const MAX_POINTS: u32 = 100_000;

fn types() {
    println!("MAX_POINTS: {}", MAX_POINTS);
    let spaces = "   ";
    let spaces = spaces.len();
    let t = true;

    let f: bool = false; // with explicit type annotation
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    // arrays in Rust have a fixed length: once declared, they cannot grow or
    // shrink in size.
    // Arrays are useful when you want your data allocated on the stack rather
    // than the heap.
    let a = [1, 2, 3, 4, 5];
    // An array is a single chunk of memory allocated on the stack
    let first = a[0];
    let second = a[1];
    another_function(5);
    let x = 5;

    // Expressions evaluate to something and make up most of the rest of the
    // code that you’ll write in Rust.
    // Note the x + 1 line without a semicolon at the end, unlike most of the
    // lines you’ve seen so far. Expressions do not include ending semicolons.
    // If you add a semicolon to the end of an expression, you turn it into a
    // statement, which will then not return a value. Keep this in mind as you
    // explore function return values and expressions next.
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    let x = five();
    println!("The value of x is: {}", x);
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index = index + 1;
    }
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

// the line let x = five(); shows that we’re using the return value of a
// function to initialize a variable. Second, the five function has no
// parameters and defines the type of the return value, but the body of the
// function is a lonely 5 with no semicolon because it’s an expression whose
// value we want to return.
fn five() -> i32 {
    5
}
