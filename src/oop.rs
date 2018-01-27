// https://doc.rust-lang.org/stable/book/second-edition/ch17-01-what-is-oo.html

// Is Rust and Object Oriented Programming Language?
// Object-oriented programs are made up of objects. An object packages both data
// and the procedures that operate on that data. The procedures are typically
// called methods or operations.
// Encapsulation: that the implementation details of an object aren’t accessible
// to code using that object.

pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }

    // We leave the list and average fields private so that there’s no way for
    // external code to add or remove items to the list field directly,
    // otherwise the average field might become out of sync when the list
    // changes.
    // If encapsulation is a required aspect for a language to be considered
    // object-oriented, then Rust meets that requirement.
}

pub fn sample() {}
