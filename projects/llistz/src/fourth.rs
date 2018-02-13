use std::rc::Rc;
use std::cell::RefCell;

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

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        assert_eq!(1, 1);
    }
}
