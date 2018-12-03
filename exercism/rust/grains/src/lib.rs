pub fn square(s: u32) -> u64 {
    match s {
        // https://doc.rust-lang.org/book/second-edition/ch18-03-pattern-syntax.html#a-bindings
        s @ 1...64 => 1u64 << (s - 1),
        _ => panic!("Square must be between 1 and 64"),
    }
}

pub fn total() -> u64 {
    u64::max_value()
}
