//! # Procedural Macros (and custom Derive)
//!
//! `hello-world-derive` is an exercise to create a simple prodedural macro to
//! provide a custom Derive trait.
//!
//! Rust includes several traits that you can derive, but it also lets
//! you define your own. We can accomplish this task through a feature
//! of Rust called "procedural macros." Eventually, procedural macros
//! will allow for all sorts of advanced metaprogramming in Rust, but
//! today, they're only for custom derive.

// Sources
// Procedural Macros: https://doc.rust-lang.org/book/first-edition/procedural-macros.html
// Commenting: https://doc.rust-lang.org/book/second-edition/ch14-02-publishing-to-crates-io.html

#[derive(Debug)]
struct DPoint {
    x: i32,
    y: i32,
}

struct Point {
    x: i32,
    y: i32,
}

use std::fmt;

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point {{ x: {}, y: {} }}", self.x, self.y)
    }
}

/// Examples
///
/// ```
/// #[macro_use]
/// extern crate hello_world_derive;
///
/// trait HelloWorld {
///     fn hello_world();
/// }
///
/// #[derive(HelloWorld)]
/// struct FrenchToast;
///
/// #[derive(HelloWorld)]
/// struct Waffles;
///
/// #[derive(HelloWorld)]
/// struct Pancakes;
///
/// fn main() {
///     FrenchToast::hello_world();
///     Waffles::hello_world();
///     Pancakes::hello_world();
/// }
/// ```
///

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
