mod types;
mod fibonacci;
mod modules;
mod to_string;
mod formatting;
mod ownership;
mod structs;
mod enums;
mod option;
mod vectors;
mod maps;
mod strings;
mod palindrome;
mod boxes_and_trees;
mod generics;
mod pointers_and_refs;
mod ref_and_ampersand;
mod traits;
mod lifetimes;

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
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        println!("it works... for now");
    }
}
