use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub struct CustomSet<T> {
    data: Vec<T>,
}

impl<T: Clone + PartialEq + Debug + Ord> CustomSet<T> {
    pub fn new(input: &[T]) -> Self {
        // meh
        let mut data: Vec<T> = input.to_owned().to_vec();
        data.sort();
        return Self { data };
    }

    pub fn contains(&self, element: &T) -> bool {
        return self.data.contains(element);
    }

    pub fn add(&mut self, element: T) {
        if !self.contains(&element) {
            self.data.push(element);
        }
        // meh
        self.data.sort();
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        return self.data.iter().all(|e| other.contains(e));
    }

    pub fn is_empty(&self) -> bool {
        return self.data.len() == 0;
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        return self.data.iter().all(|e| !other.contains(e));
    }

    pub fn intersection(&self, other: &Self) -> Self {
        return Self::new(
            &self
                .data
                .iter()
                .filter(|e| other.contains(e))
                .map(|e| e.to_owned())
                .rev()
                .collect::<Vec<_>>(),
        );
    }

    pub fn difference(&self, other: &Self) -> Self {
        Self::new(
            &self
                .data
                .iter()
                .filter(|e| !other.contains(e))
                .map(|e| e.to_owned())
                .collect::<Vec<_>>(),
        )
    }

    pub fn union(&self, other: &Self) -> Self {
        let mut d: Vec<T> = Vec::with_capacity(self.data.len() + other.data.len());
        for e in self.data.iter().chain(other.data.iter()) {
            d.push(e.to_owned());
        }
        d.sort();
        // https://doc.rust-lang.org/std/vec/struct.Vec.html#method.dedup
        // Removes consecutive repeated elements in the vector according
        // to the PartialEq trait implementation.
        //
        // If the vector is sorted, this removes all duplicates.
        d.dedup();
        return Self { data: d };
    }
}
