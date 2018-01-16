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

fn two() {
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

    // expect(self, msg: &str) -> T
    // Unwraps an option, yielding the content of a Some.
    // Panics if the value is a None with a custom panic message provided by
    // msg.
    let x = Some("value");
    assert_eq!(x.expect("the world is ending"), "value");
    // let x: Option<&str> = None;
    // x.expect("the world is ending"); // panics with `the world is ending`

    // fn unwrap_or(self, def: T) -> T
    // Returns the contained value or a default.

    assert_eq!(Some("car").unwrap_or("bike"), "car");
    assert_eq!(None.unwrap_or("bike"), "bike");

    // fn unwrap_or_else<F>(self, f: F) -> T
    // where F: FnOnce() -> T,
    // Returns the contained value or computes it from a closure.

    let k = 10;
    assert_eq!(Some(4).unwrap_or_else(|| 2 * k), 4);
    assert_eq!(None.unwrap_or_else(|| 2 * k), 20);

    // fn map<U, F>(self, f: F) -> Option<U>
    // where F: FnOnce(T) -> U,
    // Maps an Option<T> to Option<U> by applying a function to a contained value.
    // Convert an Option<String> into an Option<usize>, consuming the original:

    let maybe_some_string = Some(String::from("Hello, World!"));
    // `Option::map` takes self *by value*, consuming `maybe_some_string`
    let maybe_some_len = maybe_some_string.map(|s| s.len());
    assert_eq!(maybe_some_len, Some(13));

    // fn map_or<U, F>(self, default: U, f: F) -> U
    // where F: FnOnce(T) -> U,
    // Applies a function to the contained value (if any), or returns a default (if
    // not).

    // let x = Some("foo");
    // assert_eq!(x.map_or(42, |v| v.len()), 3);
    // let x: Option<&str> = None;
    // assert_eq!(x.map_or(42, |v| v.len()), 42);

    // fn map_or_else<U, D, F>(self, default: D, f: F) -> U
    // where D: FnOnce() -> U, F: FnOnce(T) -> U,
    // Applies a function to the contained value (if any), or computes a default (if not).

    let k = 21;
    let x = Some("foo");
    assert_eq!(x.map_or_else(|| 2 * k, |v| v.len()), 3);
    let x: Option<&str> = None;
    assert_eq!(x.map_or_else(|| 2 * k, |v| v.len()), 42);

    // fn ok_or<E>(self, err: E) -> Result<T, E>[src][−]
    // Transforms the Option<T> into a Result<T, E>, mapping Some(v) to Ok(v) and None to Err(err).
    let x = Some("foo");
    assert_eq!(x.ok_or(0), Ok("foo"));

    let x: Option<&str> = None;
    assert_eq!(x.ok_or(0), Err(0));

    // fn ok_or_else<E, F>(self, err: F) -> Result<T, E>
    // where F: FnOnce() -> E,
    // Transforms the Option<T> into a Result<T, E>, mapping Some(v) to Ok(v) and
    // None to Err(err()).

    let x = Some("foo");
    assert_eq!(x.ok_or_else(|| 0), Ok("foo"));

    let x: Option<&str> = None;
    assert_eq!(x.ok_or_else(|| 0), Err(0));

    // fn or(self, optb: Option<T>) -> Option<T>
    // Returns the option if it contains a value, otherwise returns optb.

    let x = Some(2);
    let y = None;
    assert_eq!(x.or(y), Some(2));

    let x = None;
    let y = Some(100);
    assert_eq!(x.or(y), Some(100));

    let x = Some(2);
    let y = Some(100);
    assert_eq!(x.or(y), Some(2));

    let x: Option<u32> = None;
    let y = None;
    assert_eq!(x.or(y), None);

    // fn or_else<F>(self, f: F) -> Option<T>
    // where F: FnOnce() -> Option<T>,
    // Returns the option if it contains a value, otherwise calls f and returns the result.

    fn nobody() -> Option<&'static str> {
        None
    }
    fn vikings() -> Option<&'static str> {
        Some("vikings")
    }

    assert_eq!(Some("barbarians").or_else(vikings), Some("barbarians"));
    assert_eq!(None.or_else(vikings), Some("vikings"));
    assert_eq!(None.or_else(nobody), None);

    // fn get_or_insert(&mut self, v: T) -> &mut T
    // Inserts v into the option if it is None, then returns a mutable reference to the contained value.

    let mut x = None;

    {
        let y: &mut u32 = x.get_or_insert(5);
        assert_eq!(y, &5);

        *y = 7;
    }

    assert_eq!(x, Some(7));

    // fn get_or_insert_with<F>(&mut self, f: F) -> &mut T
    // where F: FnOnce() -> T,
    // Inserts a value computed from f into the option if it is None, then returns a mutable reference to the contained value.

    let mut x = None;

    {
        let y: &mut u32 = x.get_or_insert_with(|| 5);
        assert_eq!(y, &5);

        *y = 7;
    }

    assert_eq!(x, Some(7));

    // fn take(&mut self) -> Option<T>[src][−]
    // Takes the value out of the option, leaving a None in its place.

    let mut x = Some(2);
    x.take();
    assert_eq!(x, None);

    let mut x: Option<u32> = None;
    x.take();
    assert_eq!(x, None);
}

fn three() {
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u32 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
    value_in_cents(Coin::Dime);
    value_in_cents(Coin::Quarter);
    value_in_cents(Coin::Penny);
    value_in_cents(Coin::Nickel);
}

fn four() {
    #[derive(Debug)]
    enum UsState {
        #[allow(dead_code)] Alabama,
        #[allow(dead_code)] Alaska,
        // ... etc
    }

    #[derive(Debug)]
    enum Coin {
        #[allow(dead_code)] Penny,
        #[allow(dead_code)] Nickel,
        #[allow(dead_code)] Dime,
        Quarter(UsState),
    }
    fn value_in_cents(coin: Coin) -> u32 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }

    value_in_cents(Coin::Quarter(UsState::Alaska));
}

fn five() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}

fn six() {
    let some_u8_value = Some(0u8);
    // match some_u8_value {
    //     Some(3) => println!("three"),
    //     _ => (),
    // }
    // if let takes a pattern and an expression separated by an =. It works the
    // same way as a match, where the expression is given to the match and the
    // pattern is its first arm.
    if let Some(3) = some_u8_value {
        println!("three");
    }

    let mut count = 0;
    // match coin {
    //     Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    //     _ => count += 1,
    // }
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}

pub fn sample() {
    one();
    two();
    three();
    four();
    five();
    six();
}
