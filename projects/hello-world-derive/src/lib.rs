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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
