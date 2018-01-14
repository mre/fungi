mod guess_game;
mod types;
mod fibonacci;

fn main() {
    types::sample();
    let x: u32 = fibonacci::fib(10);
    println!("fib(10) = {}", x);
    guess_game::play();
}
