mod guess_game;
mod types;
mod fibonacci;
mod modules;
mod to_string;

fn main() {
    types::sample();
    let x: u32 = fibonacci::fib(10);
    modules::try();
    println!("fib(10) = {}", x);
    to_string::sample();
    guess_game::play();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        println!("it works... for now");
    }
}
