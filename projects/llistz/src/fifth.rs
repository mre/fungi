// http://cglab.ca/~abeinges/blah/too-many-lists/book/fifth.html
// An Unsafe Singly-Linked Queue
// http://cglab.ca/~abeinges/blah/too-many-lists/book/fifth-layout.html
//
// How a stack looks like:
//
// input list:
// [Some(ptr)] -> (A, Some(ptr)) -> (B, None)
//
// stack push X:
// [Some(ptr)] -> (X, Some(ptr)) -> (A, Some(ptr)) -> (B, None)
//
// stack pop:
// [Some(ptr)] -> (A, Some(ptr)) -> (B, None)
//
// To make a queue, we just need to decide which operation to move to the end of
// the list: push, or pop? To move push to the end, we just walk all the way to
// the None and set it to Some with the new element.
//
// input list:
// [Some(ptr)] -> (A, Some(ptr)) -> (B, None)
//
// flipped push X:
// [Some(ptr)] -> (A, Some(ptr)) -> (B, Some(ptr)) -> (X, None)
//
// To move pop to the end, we just walk all the way to the node before the None,
// and take it:
//
// input list:
// [Some(ptr)] -> (A, Some(ptr)) -> (B, Some(ptr)) -> (X, None)
//
// flipped pop:
// [Some(ptr)] -> (A, Some(ptr)) -> (B, None)

// [...] because our list is singly-linked, we can't effeciently walk backwards
// in the list. To invert pop we would have to move the "tail" pointer
// backwards. But if we instead invert push we only have to move the "head"
// pointer forwards [...]

use std::mem;

pub struct List<T> {
    head: Link<T>,
    tail: Option<&mut Node<T>>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    // Function std::mem::replace
    // https://doc.rust-lang.org/std/mem/fn.replace.html
    // pub fn replace<T>(dest: &mut T, src: T) -> T
    // Replaces the value at a mutable location with a new one, returning
    // the old value, without deinitializing either one.
    //
    // use std::mem;
    //
    // let mut v: Vec<i32> = vec![1, 2];
    // let old_v = mem::replace(&mut v, vec![3, 4, 5]);
    // assert_eq!(2, old_v.len());
    // assert_eq!(3, v.len());
    //

    // pub fn push(&mut self, elem: T) {
    //     let new_tail = Box::new(Node {
    //         elem: elem,
    //         // When you push onto the tail, your next is always None
    //         next: None,
    //     });
    //     // swap the old tail to point to the new tail
    //     let old_tail = mem::replace(&mut self.tail, Some(new_tail));
    //     match old_tail {
    //         Some(mut old_tail) => {
    //             // If the old tail existed, update it to point to the new tail
    //             old_tail.next = Some(new_tail);
    //         }
    //         None => {
    //             // Otherwise, update the head to point to it
    //             self.head = Some(new_tail);
    //         }
    //     }
    // }

    // error: use of moved value: `new_tail` [E0382]
    //          old_tail.next = Some(new_tail);
    //                               ^~~~~~~~
    // note: `new_tail` moved here because it has type `Box<fifth::Node<T>>`, which is non-copyable
    //  let old_tail = mem::replace(&mut self.tail, Some(new_tail));
    //                                                   ^~~~~~~~
    // error: use of moved value: `new_tail` [E0382]
    //          self.head = Some(new_tail);
    //                           ^~~~~~~~
    // note: `new_tail` moved here because it has type `Box<fifth::Node<T>>`, which is non-copyable
    // let old_tail = mem::replace(&mut self.tail, Some(new_tail));
    //
    // use of moved value: new_tail
    // Box doesn't implement Copy, so we can't just assign it to two locations.
    // More importantly, Box owns the thing it points to, and will try to free
    // it when it's dropped. If our push implementation compiled, we'd
    // double-free the tail of our list.

    pub fn push(&mut self, elem: T) {
        let new_tail = Box::new(Node {
            elem: elem,
            // When you push onto the tail, your next is always None
            next: None,
        });

        // Put the box in the right place, and then grab a reference to its Node
        let new_tail = match self.tail.take() {
            Some(old_tail) => {
                // If the old tail existed, update it to point to the new tail
                old_tail.next = Some(new_tail);
                old_tail.next.as_mut().map(|node| &mut **node)
            }
            None => {
                // Otherwise, update the head to point to it
                self.head = Some(new_tail);
                self.head.as_mut().map(|node| &mut **node)
            }
        };

        self.tail = new_tail;
    }
}

#[cfg(test)]
mod test {
    // use super::List;

    #[test]
    fn basics() {
        // let mut list = List::new();

        assert_eq!(1, 1);
    }
}
