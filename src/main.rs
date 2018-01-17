mod guess_game;
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
    guess_game::play();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        println!("it works... for now");
    }
}
