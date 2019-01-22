pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(element: T, next: Option<Box<Node<T>>>) -> Self {
        return Self {
            data: element,
            next: next,
        };
    }
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        return Self { head: None };
    }

    pub fn len(&self) -> usize {
        let mut l: usize = 0;
        if self.head.is_none() {
            return 0;
        }
        let mut curr: Option<&Box<Node<T>>> = self.head.as_ref();
        while let Some(c) = curr {
            l += 1;
            curr = c.next.as_ref();
        }
        return l;
    }

    pub fn push(&mut self, element: T) {
        let h: Option<Box<Node<T>>> = Some(Box::new(Node::new(element, self.head.take())));
        self.head = h;
    }

    pub fn pop(&mut self) -> Option<T> {
        let head = self.head.take();
        return if let Some(mut h) = head {
            self.head = h.next.take();
            Some(h.data)
        } else {
            self.head = head;
            None
        };
    }

    pub fn peek(&self) -> Option<&T> {
        if let Some(ref h) = self.head {
            return Some(&h.data);
        } else { return None };
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        let mut l: Self = Self::new();
        let mut head = self.head.as_ref();
        while let Some(n) = head {
            l.push(n.data.clone());
            head = n.next.as_ref();
        }
        return l;
    }
}

impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(items: &[T]) -> Self {
        let mut l: Self = Self::new();
        // or items.iter().cloned()
        for item in items {
            l.push(item.clone());
        }
        return l;
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(mut self) -> Vec<T> {
        let mut v = Vec::new();
        while let Some(n) = self.pop() {
            v.insert(0, n);
        }
        return v;
    }
}
