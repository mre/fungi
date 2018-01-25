// https://doc.rust-lang.org/stable/book/second-edition/ch15-00-smart-pointers.html

// Smart Pointers
// Smart pointers are data structures that act like a pointer.
// One example that we'll explore in this chapter is the reference counting
// smart pointer type, which enables you to have multiple owners of data. The
// reference counting smart pointer keeps track of how many owners there are,
// and when there aren't any remaining, the smart pointer takes care of cleaning
// up the data.
// An additional difference between references and smart pointers is that
// references are a kind of pointer that only borrow data; by contrast, in many
// cases, smart pointers own the data that they point to.

// String and Vec<T> are actually smart pointers.
// Smart pointers are usually implemented using structs. The characteristics
// that distinguish a smart pointer from an ordinary struct are that smart
// pointers implement the Deref and Drop traits. The Deref trait allows an
// instance of the smart pointer struct to behave like a reference so that we
// can write code that works with either references or smart pointers. The Drop
// trait allows us to customize the code that gets run when an instance of the
// smart pointer goes out of scope.

// the most common smart pointers from the standard library:
// - Box<T> for allocating values on the heap;
// - Rc<T>, a reference counted type that enables multiple ownership;
// - Ref<T> and RefMut<T>, accessed through RefCell<T>, a type that enforces the
//   borrowing rules at runtime instead of compile time;

// We'll cover the interior mutability pattern where an immutable
// type exposes an API for mutating an interior value. We'll also discuss
// reference cycles, how they can leak memory, and how to prevent them.

// Box<T> Points to Data on the Heap and Has a Known Size
// Boxes allow you to store data on the heap rather than the stack. What remains
// on the stack is the pointer to the heap data.
// They're most often used in these situations:
//
// - When you have a type whose size can't be known at compile time, and you
//   want to use a value of that type in a context that needs to know an exact
//   size
// - When you have a large amount of data and you want to transfer ownership but
//   ensure the data won't be copied when you do so
// - When you want to own a value and only care that it's a type that implements
//   a particular trait rather than knowing the concrete type itself

// Using a Box<T> to store data on the heap
// how to use a box to store an i32 on the heap:
fn one() {
    // the variable b has the value of a Box that points to the value 5,
    // which is allocated on the heap
    let b = Box::new(5);
    println!("b = {}", b);
}

// Rust needs to know at compile time how much space a type takes up. One kind
// of type whose size can't be known at compile time is a recursive type where a
// value can have as part of itself another value of the same type.

// A cons list is a list where each item in the list contains two things: the
// value of the current item and the next item. The last item in the list
// contains only a value called Nil without a next item.
fn two() {
    // enum List {
    //     Cons(i32, List),
    //     Nil,
    // }
    // use List::{Cons, Nil};
    // let list = Cons(1, Cons(2, Cons(3, Nil)));
    // error[E0072]: recursive type `List` has infinite size

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    // To determine how much space to allocate for a Message value, Rust goes
    // through each of the variants to see which variant needs the most space.
    // Rust sees that Message::Quit doesn't need any space, Message::Move needs
    // enough space to store two i32 values, and so forth. Since only one
    // variant will end up being used, the most space a Message value will need
    // is the space it would take to store the largest of its variants.

    // Using Box<T> to Get a Recursive Type with a Known Size
    // help: insert indirection (e.g., a `Box`, `Rc`, or `&`) at some point to
    //     make `List` representable
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    // extern crate List;
    // use List::{Cons, Nil};
    let _list = List::Cons(1,
                           Box::new(List::Cons(2,
                                               Box::new(List::Cons(3,
                                                                   Box::new(List::Nil))))));
    // The Cons variant will need the size of an i32 plus the space to store the
    // box's pointer data. 
    // Boxes only provide the indirection and heap allocation; they don't have
    // any other special abilities; they also don't have any performance
    // overhead.
    // The Box<T> type is a smart pointer because it implements the Deref trait,
    // which allows Box<T> values to be treated like references. When a Box<T>
    // value goes out of scope, the heap data that the box is pointing to is
    // cleaned up as well because of the Box<T> type's Drop trait
    // implementation.
}

// Implementing Deref trait allows us to customize the behavior of the
// dereference operator *(as opposed to the multiplication or glob operator).
fn three() {
    // regular dereference:
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);
    
    // Using Box<T> like a reference:
    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
}

pub fn sample() {
    one();
    two();
    three();
}
