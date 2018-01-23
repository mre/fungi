// https://doc.rust-lang.org/stable/book/second-edition/ch13-02-iterators.html

// Iterators
// Definition of the Iterator Trait:
// trait Iterator {
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
//     // methods with default implementations elided
// }
//
// type Item and Self::Item, which are defining an associated type with this
// trait.
// This code says implementing the Iterator trait requires that you also define
// an Item type, and this Item type is used in the return type of the next
// method.
// The Iterator trait only requires implementors to define one method: the next
// method, which returns one item of the iterator at a time wrapped in Some and,
// when iteration is over, it returns None.

// Note that we needed to make v1_iter mutable: calling the next method on an
// iterator changes state that keeps track of where it is in the sequence. Put
// another way, this code consumes, or uses up, the iterator. Each call to next
// eats up an item from the iterator. We didn’t need to make v1_iter mutable
// when we used a for loop because the loop took ownership of v1_iter and made
// it mutable behind the scenes.
#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

pub fn sample() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }
}

// The iter method produces an iterator over immutable references. If we want to
// create an iterator that takes ownership of v1 and returns owned values, we
// can call into_iter instead of iter. Similarly, if we want to iterate over
// mutable references, we can call iter_mut instead of iter.
