pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    // First, mem::replace(&mut option, None) is such an incredibly common idiom
    // that Option actually just went ahead and made it a method: take

    // Enum std::option::Option
    // Type Option represents an optional value: every Option is either Some and
    // contains a value, or None, and does not.
    // pub enum Option<T> {
    //    None,
    //    Some(T),
    // }
    // fn take(&mut self) -> Option<T>[src][−]
    //
    // Takes the value out of the option, leaving a None in its place.
    // https://doc.rust-lang.org/std/option/enum.Option.html#method.take
    //
    // let mut x = Some(2);
    // x.take();
    // assert_eq!(x, None);
    //
    // let mut x: Option<u32> = None;
    // x.take();
    // assert_eq!(x, None);

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    // Second, match option { None => None, Some(x) => Some(y) } is such an
    // incredibly common idiom that it was called map. map takes a function to
    // execute on x in the Some(x) to produce the y in Some(y).

    // https://doc.rust-lang.org/std/option/enum.Option.html#method.map
    // fn map<U, F>(self, f: F) -> Option<U>
    //   where F: FnOnce(T) -> U,
    //
    //     Maps an Option<T> to Option<U> by applying a function to a contained value.
    //     Examples
    //     Convert an Option<String> into an Option<usize>, consuming the original:
    //
    // let maybe_some_string = Some(String::from("Hello, World!"));
    // // `Option::map` takes self *by value*, consuming `maybe_some_string`
    // let maybe_some_len = maybe_some_string.map(|s| s.len());
    //
    // assert_eq!(maybe_some_len, Some(13));

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            let node = *node;
            self.head = node.next;
            node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
