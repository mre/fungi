// https://doc.rust-lang.org/stable/book/second-edition/ch11-00-testing.html

#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }
}

pub fn sample() {}
