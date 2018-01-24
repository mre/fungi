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

    one();
}

// The iter method produces an iterator over immutable references. If we want to
// create an iterator that takes ownership of v1 and returns owned values, we
// can call into_iter instead of iter. Similarly, if we want to iterate over
// mutable references, we can call iter_mut instead of iter.

// Methods that call next are called consuming adaptors, because calling them
// uses up the iterator. One example is the sum method, which takes ownership of
// the iterator and iterates through the items by repeatedly calling next, thus
// consuming the iterator.

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
    // We aren’t allowed to use v1_iter after the call to sum since sum takes
    // ownership of the iterator we call it on.
}

// Other methods defined on the Iterator trait, known as iterator adaptors,
// allow us to change iterators into different kind of iterators.
// We can chain multiple calls to iterator adaptors to perform complex actions
// in a readable way. Because all iterators are lazy, however, we have to call
// one of the consuming adaptor methods in order to get results from calls to
// iterator adaptors.

fn one() {
    let v1: Vec<i32> = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let v2: Vec<_> = v1_iter.map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ]
    );
}

// let’s create an iterator that will only ever count from 1 to 5
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

// take the values produced by an instance of Counter, pair them with values
// produced by another Counter instance after skipping the first value, multiply
// each pair together, keep only those results that are divisible by three, and
// add all the resulting values together.
#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);
}

// Note that zip produces only four pairs; the theoretical fifth pair (5, None)
// is never produced because zip returns None when either of its input iterators
// return None.
