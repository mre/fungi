use std::rc::Rc;
use std::cell::{Ref, RefCell, RefMut};

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem: elem,
            prev: None,
            next: None,
        }))
    }
}

#[allow(dead_code)]
impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    // RefCell
    // https://doc.rust-lang.org/std/cell/struct.RefCell.html
    // A mutable memory location with dynamically checked borrow rules
    // See the module-level documentation for more.
    // https://doc.rust-lang.org/std/cell/index.html

    pub fn push_front(&mut self, elem: T) {
        let new_head = Node::new(elem);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
    }

    // Now we can use Rc::try_unwrap, which moves out the contents of an Rc out if its refcount is 1.
    //
    // Rc::try_unwrap(old_head).unwrap().into_inner().elem
    // Rc::try_unwrap returns a Result<T, Rc<T>>.

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                }
                None => {
                    self.tail.take();
                }
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }

    // pub fn peek_front(&self) -> Option<&T> {
    //     self.head.as_ref().map(|node| {
    //         &node.elem
    //     })
    // }
    //
    // pub fn peek_front(&self) -> Option<&T> {
    //     self.head.as_ref().map(|node| {
    //         &node.borrow().elem
    //     })
    // }
    //
    // error: borrowed value does not live long enough
    //     &node.borrow().elem
    //     ^~~~~~~~~~~~~
    // note: in expansion of closure expansion
    // note: expansion site
    // note: reference must be valid for the anonymous lifetime...
    // note: ...but borrowed value is only valid for the block at...
    //
    // Borrow:
    // fn borrow<'a>(&'a self) -> Ref<'a, T>
    // fn borrow_mut<'a>(&'a self) -> RefMut<'a, T>
    //
    // Ref and RefMut implement Deref and DerefMut respectively. So for most
    // intents and purposes they behave exactly like &T and &mut T. However,
    // because of how those traits work, the reference that's returned is
    // connected to the lifetime of the Ref, and not actual RefCell.
    // When a Ref gets dropped, it tells the RefCell that it's not borrowed
    // anymore. So if did manage to hold onto our reference longer than the Ref
    // existed, we could get a RefMut while a reference was [...] around and
    // [...] break Rust's type system [...].
    // We only want to return a reference, but we need to keep this Ref thing
    // around. But as soon as we return the reference from peek, the function is
    // over and the Ref goes out of scope.
    // [...] what if we just give up on totally hiding our implementation
    // details? What if we returns Refs?
    //
    // pub fn peek_front(&self) -> Option<Ref<T>> {
    //     self.head.as_ref().map(|node| {
    //         node.borrow()
    //     })
    // }
    //
    //   error: mismatched types:
    //   expected `core::option::Option<core::cell::Ref<'_, T>>`,
    //   found `core::option::Option<core::cell::Ref<'_, fourth::Node<T>>>`
    //   (expected type parameter, found struct `fourth::Node`) [E0308]
    //     self.head.as_ref().map(|node| {
    //         node.borrow()
    //     })
    //   help: run `rustc --explain E0308` to see a detailed explanation
    //
    // We have a Ref<Node<T>>, but we want a Ref<T>. We could abandon all hope
    // of encapsulation and just return that. We could also make things even
    // more complicated and wrap Ref<Node<T>> in a new type to only expose
    // access to an &T.
    //
    // Or from a nightly:
    // Make a new Ref for a component of the borrowed data.
    // (just like you can map over an Option, you can map over a Ref).
    //   map<U, F>(orig: Ref<'b, T>, f: F) -> Ref<'b, U>
    //   where F: FnOnce(&T) -> &U, U: ?Sized

    // pub fn peek_front(&self) -> Option<Ref<T>> {
    //     self.head
    //         .as_ref()
    //         .map(|node| Ref::map(node.borrow(), |node| &node.elem))
    // }
    //
    // src/fourth.rs:64:13: 64:21 error: use of unstable library feature 'cell_extras': recently added
    //     Ref::map(node.borrow(), |node| &node.elem)
    //     ^~~~~~~~
    //     note: in expansion of closure expansion
    //     note: expansion site
    //     help: add #![feature(cell_extras)] to the crate attributes to enable
    //     warning: unused import, #[warn(unused_imports)] on by default
    //     use std::cell::{Ref, RefMut, RefCell};
    //                     ^~~~~~
    // in lib.rs
    // #![feature(rc_unique, cell_extras)]

    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.elem))
    }

    pub fn push_back(&mut self, elem: T) {
        let new_tail = Node::new(elem);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_tail);
            }
            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head.take();
                }
            }
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().elem
        })
    }

    // Struct std::cell::Ref
    // https://doc.rust-lang.org/std/cell/struct.Ref.html#method.map
    //
    // fn map<U, F>(orig: Ref<'b, T>, f: F) -> Ref<'b, U>
    // where
    //     F: FnOnce(&T) -> &U,
    //     U: ?Sized,
    //
    //     Make a new Ref for a component of the borrowed data.
    //
    //     The RefCell is already immutably borrowed, so this cannot fail.
    //
    //     This is an associated function that needs to be used as
    //     Ref::map(...). A method would interfere with methods of the same name
    //     on the contents of a RefCell used through Deref.
    //
    //     Examples
    //
    // use std::cell::{RefCell, Ref};
    //
    // let c = RefCell::new((5, 'b'));
    // let b1: Ref<(u32, char)> = c.borrow();
    // let b2: Ref<u32> = Ref::map(b1, |t| &t.0);
    // assert_eq!(*b2, 5)

    // Struct std::cell::RefCell
    // https://doc.rust-lang.org/std/cell/struct.RefCell.html#method.borrow
    //
    // fn borrow(&self) -> Ref<T>
    //
    //     Immutably borrows the wrapped value.
    //     The borrow lasts until the returned Ref exits scope. Multiple
    //     immutable borrows can be taken out at the same time.
    //
    //     Panics
    //     Panics if the value is currently mutably borrowed. For a
    //     non-panicking variant, use try_borrow.
    //
    //     Examples
    //
    // use std::cell::RefCell;
    // let c = RefCell::new(5);
    // let borrowed_five = c.borrow();
    // let borrowed_five2 = c.borrow();
    //
    //     An example of panic:
    //
    // use std::cell::RefCell;
    // use std::thread;
    // let result = thread::spawn(move || {
    //     let c = RefCell::new(5);
    //     let m = c.borrow_mut();
    //     let b = c.borrow(); // this causes a panic
    // }).join();
    // assert!(result.is_err());

    pub fn peek_back(&self) -> Option<Ref<T>> {
        self.tail
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.elem))
    }

    pub fn peek_back_mut(&mut self) -> Option<RefMut<T>> {
        self.tail
            .as_ref()
            .map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.elem))
    }

    pub fn peek_front_mut(&mut self) -> Option<RefMut<T>> {
        self.head
            .as_ref()
            .map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.elem))
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.0.pop_front()
    }
}

// What if someone wants to iterate in the other direction?
// [...] DoubleEndedIterator. DoubleEndedIterator inherits from Iterator
// (meaning all DoubleEndedIterator are Iterators) and requires one new method:
// next_back. It has the exact same signature as next, but it's supposed to
// yield elements from the other end. The semantics of DoubleEndedIterator are
// [...] the iterator becomes a deque. You can consume elements from the front
// and back until the two ends converge, at which point the iterator is empty.
// [...] the best part of this interface is that it exposes the rev
// method, which wraps up the iterator to make a new one that yields the
// elements in reverse order. The semantics of this are fairly straight-forward:
// calls to next on the reversed iterator are just calls to next_back.

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        self.0.pop_back()
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop_front(), None);

        // Populate list
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_front(4);
        list.push_front(5);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);

        // ---- back -----

        // Check empty list behaves right
        assert_eq!(list.pop_front(), None);

        // Populate list
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        // Check normal removal
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_back(4);
        list.push_back(5);

        // Check normal removal
        assert_eq!(list.pop_back(), Some(5));
        assert_eq!(list.pop_back(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert!(list.peek_front().is_none());
        assert!(list.peek_back().is_none());
        assert!(list.peek_front_mut().is_none());
        assert!(list.peek_back_mut().is_none());

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(&*list.peek_front().unwrap(), &3);
        assert_eq!(&mut *list.peek_front_mut().unwrap(), &mut 3);
        assert_eq!(&*list.peek_back().unwrap(), &1);
        assert_eq!(&mut *list.peek_back_mut().unwrap(), &mut 1);
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next_back(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next(), None);
    }
}
