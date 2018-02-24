// A sample collection, that's just a wrapper over Vec<T>
#[derive(Debug)]
struct MyCollection(Vec<i32>);

// Let's give it some methods so we can create one and add things
// to it.
impl MyCollection {
    fn new() -> MyCollection {
        MyCollection(Vec::new())
    }

    fn add(&mut self, elem: i32) {
        self.0.push(elem);
    }
}

// and we'll implement IntoIterator
impl IntoIterator for MyCollection {
    type Item = i32;
    type IntoIter = ::std::vec::IntoIter<i32>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Debug)]
struct IterStuff {
    len: i32,
}

impl IterStuff {
    fn new(l: i32) -> IterStuff {
        IterStuff { len: l }
    }
}

impl IntoIterator for IterStuff {
    type Item = i32;
    type IntoIter = ::std::vec::IntoIter<i32>;

    fn into_iter(self) -> Self::IntoIter {
        // let rng: std::ops::Range<i32> = (0..self.len);
        // std::vec::Vec
        // https://doc.rust-lang.org/std/vec/struct.Vec.html#method.with_capacity
        let mut vrng: std::vec::Vec<i32> = ::std::vec::Vec::with_capacity((self.len + 1) as usize);
        for i in 0..self.len {
            vrng.push(i);
        }
        vrng.into_iter()
    }
}

// rustc scripts/custom_to_iter.rs --out-dir ./target/ && ./target/custom_to_iter
fn main() {
    println!("Now we can make a new collection...");
    let mut c = MyCollection::new();

    println!("... add some stuff to it ...");
    c.add(0);
    c.add(1);
    c.add(2);

    println!("... and then turn it into an Iterator");
    for (i, n) in c.into_iter().enumerate() {
        assert_eq!(i as i32, n);
    }

    println!("or something silly, from 9 to range");
    let r = IterStuff::new(9);
    // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.enumerate
    for (_, n) in r.into_iter().enumerate() {
        println!("{:?}", n);
    }
}

// Trait std::iter::Iterator
// https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.by_ref
//
// fn by_ref(&mut self) -> &mut Self
// Borrows an iterator, rather than consuming it.
// This is useful to allow applying iterator adaptors while still retaining
// ownership of the original iterator.
//
// Examples
// Basic usage:
//
// let a = [1, 2, 3];
// let iter = a.into_iter();
// let sum: i32 = iter.take(5)
//                    .fold(0, |acc, &i| acc + i );
// assert_eq!(sum, 6);
//
// if we try to use iter again, it won't work. The following line
// gives "error: use of moved value: `iter`
// assert_eq!(iter.next(), None);
//
// let's try that again
// instead, we add in a .by_ref()
//
// let a = [1, 2, 3];
// let mut iter = a.into_iter();
// let sum: i32 = iter.by_ref()
//                    .take(2)
//                    .fold(0, |acc, &i| acc + i );
// assert_eq!(sum, 3);
//
// now this is just fine:
// assert_eq!(iter.next(), Some(&3));
// assert_eq!(iter.next(), None);
//

// Trait std::iter::Iterator
// https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect
//
// fn collect<B>(self) -> B
// where
//     B: FromIterator<Self::Item>,
//
// Transforms an iterator into a collection.
// collect() can take anything iterable, and turn it into a relevant collection.
// This is one of the more powerful methods in the standard library, used in a
// variety of contexts.
// The most basic pattern in which collect() is used is to turn one collection
// into another. You take a collection, call iter on it, do a bunch of
// transformations, and then collect() at the end.
//
// One of the keys to collect()'s power is that many things you might not think
// of as 'collections' actually are. For example, a String is a collection of
// chars. And a collection of Result<T, E> can be thought of as single
// Result<Collection<T>, E>. See the examples below for more.
//
// Because collect() is so general, it can cause problems with type inference.
// As such, collect() is one of the few times you'll see the syntax
// affectionately known as the 'turbofish': ::<>. This helps the inference
// algorithm understand specifically which collection you're trying to collect
// into.
//
// Examples
// Basic usage:
//
// let a = [1, 2, 3];
// let doubled: Vec<i32> = a.iter()
//                          .map(|&x| x * 2)
//                          .collect();
// assert_eq!(vec![2, 4, 6], doubled);
//
// Note that we needed the : Vec<i32> on the left-hand side. This is because we
// could collect into, for example, a VecDeque<T> instead:
//
// use std::collections::VecDeque;
// let a = [1, 2, 3];
// let doubled: VecDeque<i32> = a.iter()
//                               .map(|&x| x * 2)
//                               .collect();
// assert_eq!(2, doubled[0]);
// assert_eq!(4, doubled[1]);
// assert_eq!(6, doubled[2]);
//
// Using the 'turbofish' instead of annotating doubled:
//
// let a = [1, 2, 3];
// let doubled = a.iter()
//                .map(|&x| x * 2)
//                .collect::<Vec<i32>>();
// assert_eq!(vec![2, 4, 6], doubled);
//
// Because collect() only cares about what you're collecting into, you can still
// use a partial type hint, _, with the turbofish:
//
// let a = [1, 2, 3];
// let doubled = a.iter()
//                .map(|&x| x * 2)
//                .collect::<Vec<_>>();
// assert_eq!(vec![2, 4, 6], doubled);
//
// Using collect() to make a String:
//
// let chars = ['g', 'd', 'k', 'k', 'n'];
// let hello: String = chars.iter()
//                          .map(|&x| x as u8)
//                          .map(|x| (x + 1) as char)
//                          .collect();
// assert_eq!("hello", hello);
//
// If you have a list of Result<T, E>s, you can use collect() to see if any of
// them failed:
//
// let results = [Ok(1), Err("nope"), Ok(3), Err("bad")];
// let result: Result<Vec<_>, &str> = results.iter().cloned().collect();
// // gives us the first error
// assert_eq!(Err("nope"), result);
//
// let results = [Ok(1), Ok(3)];
// let result: Result<Vec<_>, &str> = results.iter().cloned().collect();
//
// // gives us the list of answers
// assert_eq!(Ok(vec![1, 3]), result);
