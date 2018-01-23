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
