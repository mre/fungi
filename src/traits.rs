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

//  If this lib.rs is for a crate we’ve called aggregator, and someone else
//  wants to use our crate’s functionality plus implement the Summarizable trait
//  on their WeatherForecast struct, their code would need to import the
//  Summarizable trait into their scope first before they could implement it
//
// extern crate aggregator;
// use aggregator::Summarizable;

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

pub fn sample() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summary());
}
