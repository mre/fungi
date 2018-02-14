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

    // Make a new Ref for a component of the borrowed data.
    // map<U, F>(orig: Ref<'b, T>, f: F) -> Ref<'b, U>
    // where F: FnOnce(&T) -> &U,
    // U: ?Sized

    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.elem))
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        assert_eq!(1, 1);

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
    }
}
