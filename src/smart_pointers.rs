// https://doc.rust-lang.org/stable/book/second-edition/ch15-00-smart-pointers.html
// https://doc.rust-lang.org/stable/book/second-edition/ch15-01-box.html
// https://doc.rust-lang.org/stable/book/second-edition/ch15-02-deref.html
// https://doc.rust-lang.org/stable/book/second-edition/ch15-03-drop.html

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

    #[allow(dead_code)]
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

// Defining our own smart pointer
fn four() {
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    } 

    // let x = 5;
    // let y = MyBox::new(x);
    // assert_eq!(5, x);
    // assert_eq!(5, *y);
    // error: type `MyBox<{integer}>` cannot be dereferenced

    // Implementing Deref Trait defines how to treat a type like a reference.
    // The Deref trait, provided by the standard library, requires implementing
    // one method named deref that borrows self and returns a reference to the
    // inner data.

    use std::ops::Deref;

    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &T {
            // deref returns a reference to the value we want to access with the
            // * operator.
            &self.0
        }
    }

    // The type Target = T; syntax defines an associated type for this trait to
    // use. Associated types are a slightly different way of declaring a generic
    // parameter.

    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Without the Deref trait, the compiler can only dereference & references.
    // These two are equivalent:
    // *y == *(y.deref())
    // The reason the deref method returns a reference to a value, and why the
    // plain dereference outside the parentheses in *(y.deref()) is still
    // necessary, is because of ownership. If the deref method returned the
    // value directly instead of a reference to the value, the value would be
    // moved out of self.

    // Deref coercion is a convenience that Rust performs on arguments to
    // functions and methods. Deref coercion converts a reference to a type that
    // implements Deref into a reference to a type that Deref can convert the
    // original type into. Deref coercion happens automatically when we pass a
    // reference to a value of a particular type as an argument to a function or
    // method that doesn't match the type of the parameter in the function or
    // method definition, and there's a sequence of calls to the deref method
    // that will convert the type we provided into the type that the parameter
    // needs.

    // Deref coercion was added to Rust so that programmers writing function and
    // method calls don't need to add as many explicit references and
    // dereferences with & and *. This feature also lets us write more code that
    // can work for either references or smart pointers.

    fn hello(name: &str) {
        println!("Hello, {}!", name);
    }

    // Deref coercion makes it possible for us to call hello with a reference to
    // a value of type MyBox<String>.

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    // The &m dereferences to MyBox<String> m.
    // Rust can turn &MyBox<String> into &String by calling deref.
    // The standard library provides an implementation of Deref on String that
    // returns a string slice, Rust calls deref again to turn the &String into
    // &str, which matches the hello function's definition.

    // Without deref coercion this would have been the code:
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);
    // The (*m) is dereferencing the MyBox<String> into a String. Then the & and
    // [..] are taking a string slice of the String that is equal to the whole
    // string to match the signature of hello.

    // How Deref Coercion Interacts with Mutability
    //
    // Similar to how we use the Deref trait to override * on immutable
    // references, Rust provides a DerefMut trait for overriding * on mutable
    // references.
    // 
    // Rust does deref coercion when it finds types and trait implementations in
    // three cases:
    // 
    // - From &T to &U when T: Deref<Target=U>.
    // - From &mut T to &mut U when T: DerefMut<Target=U>.
    // - From &mut T to &U when T: Deref<Target=U>.
    //
    // The first two cases are the same except for mutability. The first case
    // says that if you have a &T, and T implements Deref to some type U, you
    // can get a &U transparently. The second case states that the same deref
    // coercion happens for mutable references.
    // 
    // The last case is trickier: Rust will also coerce a mutable reference to
    // an immutable one. The reverse is not possible. Because of the borrowing
    // rules, if you have a mutable reference, that mutable reference must be
    // the only reference to that data (otherwise, the program wouldn't
    // compile). Converting one mutable reference to one immutable reference
    // will never break the borrowing rules. Converting an immutable reference
    // to a mutable reference would require that there was only one immutable
    // reference to that data, and the borrowing rules don't guarantee that.
    // Therefore, Rust can't make the assumption that converting an immutable
    // reference to a mutable reference is possible.
}

// The Drop Trait runs code on Cleanup
fn five() {
    // The second trait important to the smart pointer pattern is Drop, which
    // specify the code to run when a value goes out of scope.
    // The Drop trait requires us to implement one method named drop that
    // takes a mutable reference to self.

    // A CustomSmartPointer struct whose only custom functionality is that it
    // will print out Dropping CustomSmartPointer! when the instance goes out of
    // scope.

    struct CustomSmartPointer {
        data: String,
    }

    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data `{}`!", self.data);
        }
    }

    // The Drop trait is included in the prelude, so we don’t need to import it.
    // The body of the drop function is where you’d put any logic that you
    // wanted to run when an instance of your type goes out of scope.

    let _c = CustomSmartPointer { data: String::from("my stuff") };
    let _d = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomSmartPointers created.");

    // we create a new instance of CustomSmartPointer and then print
    // out CustomSmartPointer created.. At the end of main, our instance of
    // CustomSmartPointer will go out of scope, and Rust will call the code we
    // put in the drop method, printing our final message. Note that we didn’t
    // need to call the drop method explicitly.

    // CustomSmartPointers created.
    // Dropping CustomSmartPointer with data `other stuff`!
    // Dropping CustomSmartPointer with data `my stuff`!
    
}

pub fn sample() {
    one();
    two();
    three();
    four();
    five();
}
