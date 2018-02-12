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

    // Enum std::option::Option#as_ref
    // https://doc.rust-lang.org/std/option/enum.Option.html#method.as_ref
    // impl<T> Option<T> {
    //     pub fn as_ref(&self) -> Option<&T>;
    // }
    //
    // fn as_ref(&self) -> Option<&T>
    //
    // Converts from Option<T> to Option<&T>.
    //
    // Examples
    // Convert an Option<String> into an Option<usize>, preserving the
    // original. The map method takes the self argument by value, consuming
    // the original, so this technique uses as_ref to first take an Option
    // to a reference to the value inside the original.
    //
    //    let num_as_str: Option<String> = Some("10".to_string());
    //    // First, cast `Option<String>` to `Option<&String>` with `as_ref`,
    //    // then consume *that* with `map`, leaving `num_as_str` on the stack.
    //    let num_as_int: Option<usize> = num_as_str.as_ref().map(|n| n.len());
    //    println!("still can print num_as_str: {:?}", num_as_str);
    //
    // It demotes the Option to an Option to a reference to its internals.
    pub fn peek(&self) -> Option<&T> {
        // self.head.map(|node| &node.elem)
        // Map takes self by value, which would move the Option out of the thing
        // it's in. Previously this was fine because we had just taken it out,
        // but now we actually want to leave it where it was. The correct way to
        // handle this is with the as_ref method on Option
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
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

// Tuple structs are an alternative form of struct,
// useful for trivial wrappers around other types.
pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        // access fields of a tuple struct numerically
        self.0.pop()
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

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
    }
}
