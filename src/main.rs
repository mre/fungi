#[macro_use]
extern crate lazy_static;
extern crate regex;

mod advanced;
mod boxes_and_trees;
mod closures;
mod concurrency;
mod enums;
mod fibonacci;
mod formatting;
mod generics;
mod iterators;
mod lifetimes;
mod linked_lists;
mod maps;
mod modules;
mod oop;
mod option;
mod ownership;
mod palindrome;
mod patterns;
mod pointers_and_refs;
mod ref_and_ampersand;
mod regexp;
mod smart_pointers;
mod state_machine;
mod strings;
mod structs;
mod testing;
mod to_string;
mod traits;
mod types;
mod vectors;

fn main() {
    types::sample();
    modules::sample();
    let x: u32 = fibonacci::fib(10);
    println!("fib(10) = {}", x);
    to_string::sample();
    formatting::sample();
    ownership::sample();
    structs::sample();
    enums::sample();
    option::sample();
    vectors::sample();
    maps::sample();
    strings::sample();
    palindrome::sample();
    boxes_and_trees::samples::boxes();
    boxes_and_trees::samples::left_right();
    generics::sample();
    pointers_and_refs::sample();
    ref_and_ampersand::sample();
    traits::sample();
    lifetimes::sample();
    testing::sample();
    closures::sample();
    iterators::sample();
    smart_pointers::sample();
    concurrency::sample();
    oop::sample();
    patterns::sample();
    advanced::sample();
    regexp::sample();
    linked_lists::sample();
    state_machine::sample();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        println!("it works... for now");
    }
}
