// https://doc.rust-lang.org/stable/std/option/enum.Option.html
// https://doc.rust-lang.org/stable/std/option/index.html
// Type Option represents an optional value: every Option is either Some and
// contains a value, or None, and does not.
//
// Enum std::option::Option
// enum Option<T> {
//     Some(T),
//     None,
// }

fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

fn one() {
    // The return value of the function is an option
    let result = divide(2.0, 3.0);

    // Pattern match to retrieve the value
    match result {
        // The division was valid
        Some(x) => println!("Result: {}", x),
        // The division was invalid
        None => println!("Cannot divide by 0"),
    }
}

pub fn sample() {
    one();
    // fn is_some(&self) -> bool
    // Returns true if the option is a Some value.
    let x: Option<u32> = Some(2);
    assert_eq!(x.is_some(), true);

    let x: Option<u32> = None;
    assert_eq!(x.is_some(), false);

    // fn is_none(&self) -> bool
    // Returns true if the option is a None value.
    let x: Option<u32> = Some(2);
    assert_eq!(x.is_none(), false);

    let x: Option<u32> = None;
    assert_eq!(x.is_none(), true);

    // Converts from Option<T> to Option<&mut T>.
    let mut x = Some(2);
    match x.as_mut() {
        Some(v) => *v = 42,
        None => {}
    }
    assert_eq!(x, Some(42));
}
