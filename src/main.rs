mod guess_game;
mod types;
mod fibonacci;
mod modules;

fn main() {
    types::sample();
    let x: u32 = fibonacci::fib(10);
    modules::try();
    println!("fib(10) = {}", x);
    guess_game::play();
}
