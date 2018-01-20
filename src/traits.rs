// https://doc.rust-lang.org/stable/book/second-edition/ch10-02-traits.html

pub trait Summarizable {
    fn summary(&self) -> String;
}
