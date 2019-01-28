use std::collections::{BTreeMap, BTreeSet};

pub struct School {
    grades: BTreeMap<u32, BTreeSet<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            grades: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        let students = self.grades.entry(grade).or_insert(BTreeSet::new());
        students.insert(student.to_string());
    }

    // cloned
    // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.cloned
    //
    //    fn cloned<'a, T>(self) -> Cloned<Self>
    //    where
    //      Self: Iterator<Item = &'a T>,
    //      T: 'a + Clone,
    //
    // Creates an iterator which clones all of its elements.
    //
    // This is useful when you have an iterator over &T, but you need an
    // iterator over T.

    pub fn grades(&self) -> Vec<u32> {
        return self.grades.keys().cloned().collect();
    }

    // If grade returned an `Option<&Vec<String>>`,
    // the internal implementation would be forced to keep a `Vec<String>` to lend out.
    // By returning an owned vector instead,
    // the internal implementation is free to use whatever it chooses.
    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        return self
            .grades
            .get(&grade)
            .map(|students| students.iter().cloned().collect());
    }
}
