use std::collections::BTreeMap;

// flat_map
// https://doc.rust-lang.org/std/iter/trait.Iterator.html?search=#method.flat_map
// fn flat_map<U, F>(self, f: F) -> FlatMap<Self, U, F> 
//   where
//       F: FnMut(Self::Item) -> U,
//       U: IntoIterator,
//
// Creates an iterator that works like map, but flattens nested structure.
//
// The map adapter is very useful, but only when the closure argument
// produces values. If it produces an iterator instead, there's an extra
// layer of indirection. flat_map() will remove this extra layer on its
// own.
//
// You can think of flat_map(f) as the semantic equivalent of mapping,
// and then flattening as in map(f).flatten().
//
// Another way of thinking about flat_map(): map's closure returns one
// item for each element, and flat_map()'s closure returns an iterator
// for each element.
//
// std::ascii::AsciiExt::to_ascii_uppercase
// - https://doc.rust-lang.org/std/primitive.char.html#method.to_ascii_uppercase
//
// BTreeMap implements from/to iterator.
// - https://doc.rust-lang.org/std/iter/trait.FromIterator.html
// - https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect
#[rustfmt::skip]
pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    return h                      // take the ref of the tree
        .iter()                   // take an iterator
        .flat_map(
            |(&v, ks)| {          // map on it a function on every (key, value)
                ks                // take the vector of keys
                .iter()           // take an iterator on it
                .map(move |k| {   // map a function on it (taking ownership of k)
                    return (k.to_ascii_lowercase(), v); // return the desired tuple
                })
            })
        .collect::<BTreeMap<char, i32>>();
}
