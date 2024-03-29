//! # LinkedLists
//!
//! A library with a simple example of linked lists.
//!
//! Based on [Rust By Examples - testcase linked lists](https://rustbyexample.com/custom_types/enum/testcase_linked_list.html)

use self::List::*;
use std::fmt;

/// List as a Tuple Struct
///
/// Cons: Tuple struct that wraps an element and a pointer to the next node
/// Nil: A node that signifies the end of the linked list
enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl fmt::Debug for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Cons(x, ref t) => write!(f, "list (Cons) {:?} with tail: {:?}", x, t),
            Nil => write!(f, "list (empty)"),
        }
    }
}

// Methods can be attached to an enum
#[allow(dead_code)]
#[allow(unused_variables)]
impl List {
    // Create an empty list
    fn new() -> List {
        // `Nil` has type `List`
        Nil
    }

    // Consume a list, and return the same list with a new element at its front
    fn prepend(self, elem: u32) -> List {
        // `Cons` also has type List
        Cons(elem, Box::new(self))
    }

    // returns a mutable reference to the last item (back) which will always be
    // the Nil variant.
    fn back(&mut self) -> &mut List {
        let mut node = self;

        loop {
            // https://stackoverflow.com/questions/37986640/obtaining-a-mutable-reference-by-iterating-a-recursive-structure
            // The trick here is that using {anchor} moves the content of anchor
            // into an unnamed temporary on which the match executes. Therefore,
            // in the match block we are not borrowing from anchor but from the
            // temporary, leaving us free to modify anchor.
            // See the related blog post Stuff the Identity Function Does (in Rust).
            // https://bluss.github.io/rust/fun/2015/10/11/stuff-the-identity-function-does/
            //
            // we need to transfer ownership of the mutable reference when
            // performing iteration. This is needed to ensure you never have two
            // mutable references to the same thing.
            match { node } {
                &mut Cons(_, ref mut next) => node = next,
                other => return other,
            }
        }
    }

    /// Takes a mutable reference to self and the element to append as the
    /// "last" node of the list (actually the last element will always be the
    /// Nil value of the tuple struct).
    fn append_ref(&mut self, elem: u32) {
        *self.back() = Cons(elem, Box::new(Nil));
    }

    /// Append the given element to the list.
    fn append(&mut self, elem: u32) -> &Self {
        self.append_ref(elem);
        self
    }

    // Return the length of the list
    fn len(&self) -> u32 {
        // `self` has to be matched, because the behavior of this method
        // depends on the variant of `self`
        // `self` has type `&List`, and `*self` has type `List`, matching on a
        // concrete type `T` is preferred over a match on a reference `&T`
        match *self {
            // Can't take ownership of the tail, because `self` is borrowed;
            // instead take a reference to the tail
            Cons(_, ref tail) => 1 + tail.len(),
            // Base Case: An empty list has zero length
            Nil => 0,
        }
    }

    // Return representation of the list as a (heap allocated) string
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // `format!` is similar to `print!`, but returns a heap
                // allocated string instead of printing to the console
                format!("{}, {}", head, tail.stringify())
            }
            Nil => format!("Nil"),
        }
    }
}

pub fn sample() {
    // Create an empty linked list
    let mut list = List::new();

    // Prepend some elements
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // Show the final state of the list
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}

// Rust test programs hide the stdout of successful tests in order for the test
// output to be tidy. You can disable this behavior by passing the --nocapture
// option to the test binary or to cargo test
// cargo test -- --nocapture
#[test]
fn linked_list_prepend() {
    let mut ll = List::new();
    assert_eq!(ll.len(), 0);
    ll = ll.prepend(0);
    assert_eq!(ll.len(), 1);
    ll = ll.prepend(1);
    assert_eq!(ll.len(), 2);
    assert_eq!(ll.stringify(), String::from("1, 0, Nil"));
    println!("ll is {:?}", ll);
    println!("back is {:?}", ll.back());
    let ll = ll.append(2);
    assert_eq!(ll.len(), 3);
    assert_eq!(ll.stringify(), String::from("1, 0, 2, Nil"));
}
