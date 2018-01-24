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
// of type whose size can’t be known at compile time is a recursive type where a
// value can have as part of itself another value of the same type.

// A cons list is a list where each item in the list contains two things: the
// value of the current item and the next item. The last item in the list
// contains only a value called Nil without a next item.
fn two() {
    enum List {
        Cons(i32, List),
        Nil,
    }
    // use List::{Cons, Nil};
    let list = Cons(1, Cons(2, Cons(3, Nil)));
    // error[E0072]: recursive type `List` has infinite size

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
}

pub fn sample() {
    one();
}
