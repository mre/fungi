// https://doc.rust-lang.org/std/collections/binary_heap/struct.BinaryHeap.html

use std::fmt;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

fn max_heap() {
    // Type inference lets us omit an explicit type signature (which
    // would be `BinaryHeap<i32>` in this example).
    let mut heap = BinaryHeap::new();

    // We can use peek to look at the next item in the heap. In this case,
    // there's no items in there yet so we get None.
    assert_eq!(heap.peek(), None);

    // Let's add some scores...
    heap.push(1);
    heap.push(5);
    heap.push(2);

    // Now peek shows the most important item in the heap.
    assert_eq!(heap.peek(), Some(&5));

    // We can check the length of a heap.
    assert_eq!(heap.len(), 3);

    // We can iterate over the items in the heap, although they are returned in
    // a random order.
    for x in &heap {
        println!("{}", x);
    }

    // If we instead pop these scores, they should come back in order.
    assert_eq!(heap.pop(), Some(5));
    assert_eq!(heap.pop(), Some(2));
    assert_eq!(heap.pop(), Some(1));
    assert_eq!(heap.pop(), None);

    // We can clear the heap of any remaining items.
    heap.clear();

    // The heap should now be empty.
    assert!(heap.is_empty())
}

// Thing have a priority, an ordering, where the lower is the greater.
#[derive(Eq, PartialEq, Debug)]
struct Thing {
    content: usize,
}

impl fmt::Display for Thing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.content)
    }
}

// Trait std::cmp::Ord
// https://doc.rust-lang.org/std/cmp/trait.Ord.html
// fn cmp(&self, other: &Self) -> Ordering

impl Ord for Thing {
    fn cmp(&self, other: &Thing) -> Ordering {
        other
            .content
            .cmp(&self.content)
            .then_with(|| Ordering::Equal)
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Thing {
    fn partial_cmp(&self, other: &Thing) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn min_heap() {
    let mut heap = BinaryHeap::new();

    assert_eq!(heap.peek(), None);

    let a: &Thing = &Thing { content: 0 };
    let b: &Thing = &Thing { content: 1 };
    let c: &Thing = &Thing { content: 2 };

    heap.push(c);
    heap.push(a);
    heap.push(b);

    assert_eq!(heap.peek(), Some(&a));

    for x in &heap {
        println!("{}", x);
    }

    heap.clear();

    assert!(heap.is_empty())
}

// #[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    pub fn things_are_comparable() {
        let a: Thing = Thing { content: 0 };
        let b: Thing = Thing { content: 1 };

        assert_eq!(&a.cmp(&b), &Ordering::Greater);
        assert_eq!(&b.cmp(&a), &Ordering::Less);
        assert_eq!(&a.cmp(&a), &Ordering::Equal);
    }

    // #[test]
    pub fn things_are_pushed_in_the_heap() {
        let a: &Thing = &Thing { content: 0 };
        let b: &Thing = &Thing { content: 1 };
        let c: &Thing = &Thing { content: 2 };

        let mut heap = BinaryHeap::new();

        assert_eq!(heap.peek(), None);

        heap.push(a);
        heap.push(b);
        heap.push(c);

        assert_eq!(heap.peek(), Some(&a));
        assert_eq!(heap.len(), 3);

        for x in &heap {
            println!("{}", x);
        }

        assert_eq!(heap.pop(), Some(a));
        assert_eq!(heap.pop(), Some(b));
        assert_eq!(heap.pop(), Some(c));
        assert_eq!(heap.pop(), None);

        heap.clear();

        assert!(heap.is_empty())
    }

    //  #[test]
    #[allow(dead_code)]
    pub fn things_explode() {
        panic!("boom");
    }
}

// rustc scripts/binary_heap.rs --out-dir ./target && ./target/binary_heap
fn main() {
    tests::things_are_comparable();
    tests::things_are_pushed_in_the_heap();

    max_heap();
    println!("---");
    min_heap()
}
