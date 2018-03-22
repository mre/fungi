// https://users.rust-lang.org/t/how-to-solve-borrow-self-as-mutable-more-than-once-in-this-case/16350

use std::collections::HashMap;

fn main() {
    let mut d = Data::default();
    d.mutate();
}

#[derive(Default)]
struct Cache {
    field: i32
}

#[derive(Default)]
struct Data {
    map: HashMap<i32, Cache>
}

// impl Data {
//     fn mutate(&mut self) -> () {
//         let c = self.map.entry(0).or_insert_with(|| Cache::default());
//         if self.read_only_helper() {
//             c.field = 1;
//         }
//     }
//     fn read_only_helper(&mut self)->bool {
//         self.map.is_empty()
//     }
// }

// A way to work around this in this particular case would be to get the
// check if the map is empty out of the way before you borrow the entry out
// of the map. This is technically a logical change, but what you have
// doesn’t really make much sense because even if your map is empty you’re
// or_insert_with(...)'ing so it couldn’t possibly still be empty when you
// are trying to check.

// Code for what I mean:

impl Data {
    fn mutate(&mut self) -> () {
        let map_is_empty = self.map.is_empty();
        let c = self.map.entry(0).or_insert_with(|| Cache::default());
        if map_is_empty {
            c.field = 1;
        }
    }
}

// I can not change the order (although in this specific example it is
// possible). I need to obtain cache entry from self, and continue to use
// self (i.e. call it’s methods) and use cache entry (to mutate it) in
// parallel. This is going to be difficult because in accordance with the
// encapsulation principle, using any method of self is understood by the
// compiler to borrow and potentially access all of it, including the inner
// cache to which you are holding a mutable reference.
// There are two common strategies for solving this kind of issue:
// Drop one layer of abstraction lower, and use fields directly instead of
// calling object-wide methods. This exposes the disjointness of the
// underlying borrows (there are periodical discussions about how we could
// extend method interfaces to similarly expose disjointness, nothing
// concrete yet though) Move the cache outside of self (there are various
// possibilities for this, including moving it on the heap with Rc).

    
