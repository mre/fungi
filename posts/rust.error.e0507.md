You tried to move out of a value which was borrowed. Erroneous code example:

```
use std::cell::RefCell;

struct TheDarkKnight;

impl TheDarkKnight {
    fn nothing_is_true(self) {}
}

fn main() {
    let x = RefCell::new(TheDarkKnight);

    x.borrow().nothing_is_true(); // error: cannot move out of borrowed content
}
```

Here, the `nothing_is_true` method takes the ownership of `self`. However,
`self` cannot be moved because `.borrow()` only provides an `&TheDarkKnight`,
which is a borrow of the content owned by the `RefCell`. To fix this error,
you have three choices:

* Try to avoid moving the variable.
* Somehow reclaim the ownership.
* Implement the `Copy` trait on the type.

Examples:

```
use std::cell::RefCell;

struct TheDarkKnight;

impl TheDarkKnight {
    fn nothing_is_true(&self) {} // First case, we don't take ownership
}

fn main() {
    let x = RefCell::new(TheDarkKnight);

    x.borrow().nothing_is_true(); // ok!
}
```

Or:

```
use std::cell::RefCell;

struct TheDarkKnight;

impl TheDarkKnight {
    fn nothing_is_true(self) {}
}

fn main() {
    let x = RefCell::new(TheDarkKnight);
    let x = x.into_inner(); // we get back ownership

    x.nothing_is_true(); // ok!
}
```

[Struct std::cell::RefCell](https://doc.rust-lang.org/std/cell/struct.RefCell.html#method.into_inner)

```
pub fn into_inner(self) -> T
```

Consumes the RefCell, returning the wrapped value.

Examples

```
use std::cell::RefCell;
let c = RefCell::new(5);
let five = c.into_inner();
```

Or:

```
use std::cell::RefCell;

#[derive(Clone, Copy)] // we implement the Copy trait
struct TheDarkKnight;

impl TheDarkKnight {
    fn nothing_is_true(self) {}
}

fn main() {
    let x = RefCell::new(TheDarkKnight);

    x.borrow().nothing_is_true(); // ok!
}
```

Moving a member out of a mutably borrowed struct will also cause `E0507` error:

```
struct TheDarkKnight;

impl TheDarkKnight {
    fn nothing_is_true(self) {}
}

struct Batcave {
    knight: TheDarkKnight
}

fn main() {
    let mut cave = Batcave {
        knight: TheDarkKnight
    };
    let borrowed = &mut cave;

    borrowed.knight.nothing_is_true(); // E0507
}
```

It is fine only if you put something back. `mem::replace` can be used for that:

```
use std::mem;

let mut cave = Batcave {
    knight: TheDarkKnight
};
let borrowed = &mut cave;

mem::replace(&mut borrowed.knight, TheDarkKnight).nothing_is_true(); // ok!
```

[Function std::mem::replace](https://doc.rust-lang.org/std/mem/fn.replace.html)

```
pub fn replace<T>(dest: &mut T, src: T) -> T
```

Replaces the value at a mutable location with a new one, returning the
old value, without deinitializing either one.

Examples

A simple example:

```
use std::mem;

let mut v: Vec<i32> = vec![1, 2];

let old_v = mem::replace(&mut v, vec![3, 4, 5]);
assert_eq!(2, old_v.len());
assert_eq!(3, v.len());
```

`replace` allows consumption of a struct field by replacing it with
another value. Without `replace` you can run into issues like these:

```
// This does not compile

struct Buffer<T> { buf: Vec<T> }

impl<T> Buffer<T> {
    fn get_and_reset(&mut self) -> Vec<T> {
        // error: cannot move out of dereference of `&mut`-pointer
        let buf = self.buf;
        self.buf = Vec::new();
        buf
    }
}
```

Note that `T` does not necessarily implement `Clone`, so it can't even clone
and reset `self.buf`. But `replace` can be used to disassociate the original
value of `self.buf` from `self`, allowing it to be returned:

```
use std::mem;

impl<T> Buffer<T> {
    fn get_and_reset(&mut self) -> Vec<T> {
        mem::replace(&mut self.buf, Vec::new())
    }
}
```

You can find more information about borrowing in the rust-book:
http://doc.rust-lang.org/book/first-edition/references-and-borrowing.html
