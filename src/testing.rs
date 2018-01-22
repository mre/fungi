// https://doc.rust-lang.org/stable/book/second-edition/ch11-00-testing.html

#[allow(dead_code)]
#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    #[allow(dead_code)]
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

#[allow(dead_code)]
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[allow(dead_code)]
pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[allow(dead_code)]
pub struct Guess {
    value: u32,
}

impl Guess {
    #[allow(dead_code)]
    pub fn new(value: u32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}

#[allow(dead_code)]
fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

// argo test -- --test-threads=4
// argo test -- --test-threads=1
// cargo test -- --nocapture
// cargo test one_hundred
// cargo test add
// cargo test -- --ignored
//
// Unit tests
// We put unit tests in the src directory, in each file with the code that
// they’re testing. The convention is that we create a module named tests in
// each file to contain the test functions, and we annotate the module with
// cfg(test).
// The #[cfg(test)] annotation on the tests module tells Rust to compile and run
// the test code only when we run cargo test, and not when we run cargo build.
// We use #[cfg(test)]to specify that they should not be included in the
// compiled result.
//
// Rust’s privacy rules do allow you to test private functions.
//
// Integration tests
// To write integration tests for our code, we need to make a tests directory at
// the top level of our project directory.

#[cfg(test)]
mod tests {
    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        // panic!("Make this test fail");
    }

    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            length: 8,
            width: 7,
        };
        let smaller = Rectangle {
            length: 5,
            width: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            length: 8,
            width: 7,
        };
        let smaller = Rectangle {
            length: 5,
            width: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
    // Note that in some languages and test frameworks, the parameters to the
    // functions that assert two values are equal are called expected and actual
    // and the order in which we specify the arguments matters. However, in
    // Rust, they’re called left and right instead, and the order in which we
    // specify the value we expect and the value that the code under test
    // produces doesn’t matter. We could write the assertion in this test as
    // assert_eq!(add_two(2), 4), which would result in a failure message that
    // says assertion failed: `(left == right)` (left: `5`, right: `4`).

    // Under the surface, the assert_eq! and assert_ne! macros use the operators
    // == and !=, respectively. When the assertions fail, these macros print
    // their arguments using debug formatting, which means the values being
    // compared must implement the PartialEq and Debug traits.

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100_with_message() {
        Guess::new(200);
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }

    // integration tests
    // We’ve added extern crate adder at the top, which we didn’t need in the
    // unit tests. This is because each test in the tests directory is an
    // entirely separate crate, so we need to import our library into each of
    // them. We don’t need to annotate any code in tests/integration_test.rs
    // with #[cfg(test)].
    // {
    //   tests/integration_test.rs
    //   extern crate adder;
    //   #[test]
    //   fn it_adds_two() {
    //       assert_eq!(4, adder::add_two(2));
    //   }
    // }

    // As you add more integration tests, you may want to make more than one
    // file in the tests directory to help organize them; for example, to group
    // the test functions by the functionality they’re testing. As we mentioned,
    // each file in the tests directory is compiled as its own separate crate.

    // declaration of a common module to run as setup for the tests
    //
    // FindMe in tests/common/mod.rs
    // pub fn setup() {
    //   // ... setup code specific to your library's tests would go here
    // }
    // extern crate adder;
    // mod common;
    // #[test]
    // fn it_adds_two() {
    //     common::setup();
    //     assert_eq!(4, adder::add_two(2));
    // }

}

pub fn sample() {}
