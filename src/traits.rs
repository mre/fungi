// https://doc.rust-lang.org/stable/book/second-edition/ch10-02-traits.html

pub trait Summarizable {
    fn summary(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summarizable for NewsArticle {
    fn summary(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summarizable for Tweet {
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

//  If this lib.rs is for a crate we've called aggregator, and someone else
//  wants to use our crate's functionality plus implement the Summarizable trait
//  on their WeatherForecast struct, their code would need to import the
//  Summarizable trait into their scope first before they could implement it
//
// extern crate aggregator;
// use aggregator::Summarizable;

#[allow(dead_code)]
struct WeatherForecast {
    high_temp: f64,
    low_temp: f64,
    chance_of_precipitation: f64,
}

impl Summarizable for WeatherForecast {
    fn summary(&self) -> String {
        format!(
            "The high will be {}, and the low will be {}. The chance of
        precipitation is {}%.",
            self.high_temp, self.low_temp, self.chance_of_precipitation
        )
    }
}

// One restriction to note with trait implementations: we may implement a trait
// on a type as long as either the trait or the type are local to our crate. In
// other words, we aren't allowed to implement external traits on external
// types. We can't implement the Display trait on Vec, for example, since both
// Display and Vec are defined in the standard library. We are allowed to
// implement standard library traits like Display on a custom type like Tweet as
// part of our aggregator crate functionality. We could also implement
// Summarizable on Vec in our aggregator crate, since we've defined Summarizable
// there. This restriction is part of what's called the orphan rule, which you
// can look up if you're interested in type theory. Briefly, it's called the
// orphan rule because the parent type is not present. Without this rule, two
// crates could implement the same trait for the same type, and the two
// implementations would conflict: Rust wouldn't know which implementation to
// use. Because Rust enforces the orphan rule, other people's code can't break
// your code and vice versa.

pub trait SummarizableWithDefault {
    fn summary(&self) -> String {
        String::from("(Read more...)")
    }
}

#[allow(dead_code)]
pub struct NewsArticleSummary {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl SummarizableWithDefault for NewsArticleSummary {}

pub trait SummarizableWithAuthorSummary {
    fn author_summary(&self) -> String;

    fn summary(&self) -> String {
        format!("(Read more from {}...)", self.author_summary())
    }
}

pub struct TweetWithAuthorSummary {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl SummarizableWithAuthorSummary for TweetWithAuthorSummary {
    fn author_summary(&self) -> String {
        format!("@{}", self.username)
    }
}

#[allow(dead_code)]
pub fn notify<T: Summarizable>(item: T) {
    println!("Breaking news! {}", item.summary());
}

use std::fmt::{Debug, Display};

#[allow(dead_code)]
fn some_function<T: Display + Clone, U: Clone + Debug>(_t: T, _u: U) -> i32 {
    0
}

#[allow(dead_code)]
fn some_other_function<T, U>(_t: T, _u: U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    0
}

fn one() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summary());

    let tweet = TweetWithAuthorSummary {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summary());
    // 1 new tweet: (Read more from @horse_ebooks...)
}

fn two() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best
    hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summary());
}

fn three() {
    use std::fmt::Display;

    #[allow(dead_code)]
    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        #[allow(dead_code)]
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    impl<T: Display + PartialOrd> Pair<T> {
        #[allow(dead_code)]
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }
}

pub fn sample() {
    one();
    two();
    three();
}
