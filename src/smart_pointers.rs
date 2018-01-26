// https://doc.rust-lang.org/stable/book/second-edition/ch15-00-smart-pointers.html
// https://doc.rust-lang.org/stable/book/second-edition/ch15-01-box.html
// https://doc.rust-lang.org/stable/book/second-edition/ch15-02-deref.html
// https://doc.rust-lang.org/stable/book/second-edition/ch15-03-drop.html
// https://doc.rust-lang.org/stable/book/second-edition/ch15-04-rc.html
// https://doc.rust-lang.org/stable/book/second-edition/ch15-05-interior-mutability.html

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

fn five() {
    // The Drop trait is included in the prelude, so we don't need to import it.
    // The body of the drop function is where you'd put any logic that you
    // wanted to run when an instance of your type goes out of scope.

    let _c = CustomSmartPointer { data: String::from("my stuff") };
    let _d = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomSmartPointers created.");

    // we create a new instance of CustomSmartPointer and then print
    // out CustomSmartPointer created.. At the end of main, our instance of
    // CustomSmartPointer will go out of scope, and Rust will call the code we
    // put in the drop method, printing our final message. Note that we didn't
    // need to call the drop method explicitly.

    // CustomSmartPointers created.
    // Dropping CustomSmartPointer with data `other stuff`!
    // Dropping CustomSmartPointer with data `my stuff`!
}

// Drop a value early with std::mem::drop
// One example is when using smart pointers that manage locks; you may want to
// force the drop method that releases the lock to run so that other code in the
// same scope can acquire the lock.
fn six() {
    // let c = CustomSmartPointer { data: String::from("some data") };
    // println!("CustomSmartPointer created.");
    // c.drop();
    // error[E0040]: explicit use of destructor method
    // println!("CustomSmartPointer dropped before the end of main.");
    // this would be a double free error since Rust would be trying to clean up
    // the same value twice.
    //
    // The std::mem::drop function is different than the drop method in the Drop
    // trait. We call it by passing the value we want to force to be dropped
    // early as an argument. std::mem::drop is in the prelude.
    let c = CustomSmartPointer { data: String::from("some data") };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}

// In order to enable multiple ownership, Rust has a type called Rc<T>. Its
// name is an abbreviation for reference counting. Reference counting means
// keeping track of the number of references to a value in order to know if
// a value is still in use or not. If there are zero references to a value,
// the value can be cleaned up without any references becoming invalid.
//
// Rc<T> is used when we want to allocate some data on the heap for multiple
// parts of our program to read, and we can't determine at compile time which
// part will finish using the data last.
//
// two lists that both share ownership of a third list.
fn seven() {
    #[allow(dead_code)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    // use List::{Cons, Nil};
    {
        let _a = List::Cons(5, Box::new(List::Cons(10, Box::new(List::Nil))));
        // let b = Cons(3, Box::new(a));
        // let c = Cons(4, Box::new(a));
        // error[E0382]: use of moved value: `a`
    }
}

fn eight() {
    // We could change the definition of Cons to hold references instead, but
    // then we'd have to specify lifetime parameters. By specifying lifetime
    // parameters, we'd be specifying that every element in the list will live
    // at least as long as the list itself. The borrow checker wouldn't let us
    // compile let a = Cons(10, &Nil); for example, since the temporary Nil
    // Value would be dropped before a could take a reference to it.
    // Instead, we'll change our definition of List to use Rc<T> in place of
    // Box<T>. Every time we call Rc::clone, the reference count to the data
    // within the Rc is increased, and the data won't be cleaned up unless there
    // are zero references to it:
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }

    // use List::{Cons, Nil};
    use std::rc::Rc;

    let a = Rc::new(List::Cons(5, Rc::new(List::Cons(10, Rc::new(List::Nil)))));
    let _b = List::Cons(3, Rc::clone(&a));
    let _c = List::Cons(4, Rc::clone(&a));

    let a = Rc::new(List::Cons(5, Rc::new(List::Cons(10, Rc::new(List::Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let _b = List::Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let _c = List::Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    // Rc<T> allows us to share data between multiple parts of our program for
    // reading only, via immutable references. If Rc<T> allowed us to have
    // multiple mutable references too, we'd be able to violate one of the the
    // borrowing rules: multiple mutable borrows to the same place can cause
    // data races and inconsistencies.
}

// RefCell<T> and the Interior Mutability Pattern
//
// Interior mutability is a design pattern in Rust for allowing you to mutate
// data even when there are immutable references to that data, normally
// disallowed by the borrowing rules. To do so, the pattern uses unsafe code
// inside a data structure to bend Rust's usual rules around mutation and
// borrowing.
//
// Enforcing Borrowing Rules at Runtime with RefCell<T>
// Unlike Rc<T>, the RefCell<T> type represents single ownership over the data
// it holds. So, what makes RefCell<T> different than a type like Box<T>? Let's
// recall the borrowing rules:
//
// At any given time, you can have either but not both of:
// One mutable reference.
// Any number of immutable references.
// References must always be valid.
//
// With references and Box<T>, the borrowing rules' invariants are enforced at
// compile time. With RefCell<T>, these invariants are enforced at runtime. With
// references, if you break these rules, you'll get a compiler error. With
// RefCell<T>, if you break these rules, you'll get a panic!.
//
// To recap the reasons to choose Box<T>, Rc<T>, or RefCell<T>:
//
// - Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have
//   single owners.
// - Box<T> allows immutable or mutable borrows checked at compile time; Rc<T>
//   only allows immutable borrows checked at compile time; RefCell<T> allows
//   immutable or mutable borrows checked at runtime.
// - Because RefCell<T> allows mutable borrows checked at runtime, we can mutate
//   the value inside the RefCell<T> even when the RefCell<T> is itself
//   immutable.
// - The last reason is the interior mutability pattern.

// A consequence of the borrowing rules is that when we have an immutable
// value, we can't borrow it mutably. For example, this code won't compile:
//   let x = 5;
//   let y = &mut x;
// If we try to compile this, we'll get this error:
// error[E0596]: cannot borrow immutable local variable `x` as mutable
//
// However, there are situations where it would be useful for a value to be
// able to mutate itself in its methods, but to other code, the value would
// appear to be immutable. Code outside the value's methods would not be
// able to mutate the value. RefCell<T> is one way to get the ability to
// have interior mutability.

pub trait Messenger {
    fn send(&self, msg: &str);
}

#[allow(dead_code)]
pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where T: Messenger {
    #[allow(dead_code)]
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    #[allow(dead_code)]
    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 0.75 && percentage_of_max < 0.9 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        } else if percentage_of_max >= 0.9 && percentage_of_max < 1.0 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        }
    }
}

// the Messenger trait has one method, send, that takes an immutable
// reference to self and text of the message. we want to test the behavior
// of the set_value method on the LimitTracker.
// What we need is a mock object that, instead of actually sending an email
// or text message when we call send, will only keep track of the messages
// it's told to send.

#[cfg(test)]
mod tests {
    use super::*;

    // struct MockMessenger {
    //     sent_messages: Vec<String>,
    // }
    //
    // impl MockMessenger {
    //     fn new() -> MockMessenger {
    //         MockMessenger { sent_messages: vec![] }
    //     }
    // }
    //
    // // This implementation is not accepted by the compiler:
    // impl Messenger for MockMessenger {
    //     fn send(&self, message: &str) {
    //         self.sent_messages.push(String::from(message));
    //     }
    // }
    // error[E0596]: cannot borrow immutable field `self.sent_messages` as mutable
    //    --> src/lib.rs:46:13
    //    |
    // 45 |         fn send(&self, message: &str) {
    //    |                 ----- use `&mut self` here to make mutable
    // 46 |             self.sent_messages.push(String::from(message));
    //    |             ^^^^^^^^^^^^^^^^^^ cannot mutably borrow immutable field

    // This is where interior mutability can help! We're going to store the
    // sent_messages within a RefCell, and then the send message will be
    // able to modify sent_messages to store the messages we've seen.

    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger { sent_messages: RefCell::new(vec![]) }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    // When creating immutable and mutable references we use the & and &mut
    // syntax, respectively. With RefCell<T>, we use the borrow and borrow_mut
    // methods, which are part of the safe API that belongs to RefCell<T>. The
    // borrow method returns the smart pointer type Ref, and borrow_mut returns
    // the smart pointer type RefMut. Both types implement Deref so we can treat
    // them like regular references.

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }

    struct MockMessengerPanic {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessengerPanic {
        fn new() -> MockMessengerPanic {
            MockMessengerPanic { sent_messages: RefCell::new(vec![]) }
        }
    }

    // Creating two mutable references in the same scope to see that RefCell<T>
    // will panic
    impl Messenger for MockMessengerPanic {
        fn send(&self, message: &str) {
            let mut one_borrow = self.sent_messages.borrow_mut();
            let mut two_borrow = self.sent_messages.borrow_mut();

            one_borrow.push(String::from(message));
            two_borrow.push(String::from(message));
        }
    }

    #[test]
    #[should_panic(expected = "already borrowed: BorrowMutError")]
    fn it_panics_before_sending_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessengerPanic::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}

// A common way to use RefCell<T> is in combination with Rc<T>. Recall that
// Rc<T> lets us have multiple owners of some data, but it only gives us
// immutable access to that data. If we have an Rc<T> that holds a RefCell<T>,
// then we can get a value that can have multiple owners and that we can mutate.
//
// using a RefCell<T> in the Cons definition, we're allowed to modify the value
// stored in all the lists
#[derive(Debug)]
enum ListRefCell {
    Cons(Rc<RefCell<i32>>, Rc<ListRefCell>),
    Nil,
}

use self::ListRefCell::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;

fn nine() {
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    // We create a value that's an instance of Rc<RefCell<i32> and store it in a
    // variable named value so we can access it directly later.
    *value.borrow_mut() += 10;
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
    // a after = Cons(RefCell { value: 15 }, Nil)
    // b after = Cons(RefCell { value: 6 }, Cons(RefCell { value: 15 }, Nil))
    // c after = Cons(RefCell { value: 10 }, Cons(RefCell { value: 15 }, Nil))
}

// The standard library has other types that provide interior mutability, too,
// like Cell<T>, which is similar except that instead of giving references to
// the inner value, the value is copied in and out of the Cell<T>. There's also
// Mutex<T>, which offers interior mutability that's safe to use across threads.

// Reference cycles can leak memory
#[derive(Debug)]
enum ListLeak {
    Cons(i32, RefCell<Rc<ListLeak>>),
    Nil,
}

fn ten() {
    use std::rc::Rc;
    use std::cell::RefCell;
    use self::ListLeak::{Cons, Nil};

    impl ListLeak {
        // The second element in the Cons variant is now RefCell<Rc<List>>,
        // meaning that instead of having the ability to modify the i32 value
        // like we did in Listing 15-19, we want to be able to modify which List
        // a Cons variant is pointing to. We've also added a tail method to make
        // it convenient for us to access the second item, if we have a Cons
        // variant.
        fn tail(&self) -> Option<&RefCell<Rc<ListLeak>>> {
            match *self {
                Cons(_, ref item) => Some(item),
                Nil => None,
            }
        }
    } 

    // This code creates a list in a, a list in b that points to the list in a,
    // and then modifies the list in a to point to b, which creates a reference
    // cycle. There are println! statements along the way to show what the
    // reference counts are at various points in this process.

    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(ref link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // a initial rc count = 1
    // a next item = Some(RefCell { value: Nil })
    // a rc count after b creation = 2
    // b initial rc count = 1
    // b next item = Some(RefCell { value: Cons(5, RefCell { value: Nil }) })
    // b rc count after changing a = 2
    // a rc count after changing a = 2

    // Uncomment the next line to see that we have a cycle; it will
    // overflow the stack
    // println!("a next item = {:?}", a.tail());
}

fn eleven() {
    // We can also create a weak reference to the value within an Rc instance by
    // calling Rc::downgrade and passing a reference to the Rc. When we call
    // Rc::downgrade, we get a smart pointer of type Weak<T>. The Rc type uses
    // weak_count to keep track of how many Weak<T> references exist.
    // The difference is the weak_count does not need to be 0 in order for the
    // Rc instance to be cleaned up.
    //
    // Because the value that Weak<T> references might have been dropped, in
    // order to do anything with the value that a Weak<T> is pointing to, we
    // have to check to make sure the value is still around. We do this by
    // calling the upgrade method on a Weak<T> instance, which will return an
    // Option<Rc<T>>.
}

pub fn sample() {
    one();
    two();
    three();
    four();
    five();
    six();
    seven();
    eight();
    nine();
    ten();
}
