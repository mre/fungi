#[derive(Debug)]
struct StateMachine {
    st: String,
}

type FromStateMachineToStateFn = fn(&mut StateMachine) -> StateFn;

// a tuple struct with one (implicit) field: a function that takes a mutable
// reference to a StateMachine and returns another StateFn.
struct StateFn(FromStateMachineToStateFn);

impl StateMachine {
    // Returns the next state from the start state: foo
    fn start(&mut self) -> StateFn {
        self.st = String::from("start");
        StateFn(Self::foo)
    }

    fn foo(&mut self) -> StateFn {
        self.st = String::from("foo");
        StateFn(Self::end)
    }

    fn end(&mut self) -> StateFn {
        self.st = String::from("end");
        StateFn(Self::end)
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
impl StateFn {
    fn step(&self, sm: &mut StateMachine) -> StateFn {
        self.0(sm)
    }
}

// The Deref trait, provided by the standard library, requires implementing one
// method named deref that borrows self and returns a reference to the inner
// data.
use std::ops::Deref;

// Without the Deref trait, the compiler can only dereference & references. The
// Deref trait's deref method gives the compiler the ability to take a value of
// any type that implements Deref and call the deref method in order to get a &
// reference that it knows how to dereference.
// https://doc.rust-lang.org/stable/book/second-edition/ch15-02-deref.html
//
// Using Box<T> Like a Reference
// let x = 5;
// let y = &x;
// let y = Box::new(x);
// let y = MyBox::new(x);
// assert_eq!(5, x);
// assert_eq!(5, *y);
//
// struct MyBox<T>(T);
// impl<T> MyBox<T> {
//     fn new(x: T) -> MyBox<T> {
//         MyBox(x)
//     }
// }
// use std::ops::Deref;
// impl<T> Deref for MyBox<T> {
//     type Target = T;
//
//     fn deref(&self) -> &T {
//         &self.0
//     }
// }
impl Deref for StateFn {
    // Target is an associated type:
    // https://doc.rust-lang.org/stable/book/second-edition/ch19-03-advanced-traits.html#associated-types
    // Associated types are a way of associating a type placeholder with a trait
    // such that the trait method definitions can use these placeholder types in
    // their signatures. The implementor of a trait will specify the concrete
    // type to be used in this type's place for the particular implementation.
    type Target = FromStateMachineToStateFn;

    // with &self.0 so that deref returns a reference to the value we want to
    // access with the * operator.
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn sample() {
    let mut state_machine = StateMachine {
        st: String::from(""),
    };

    let mut state = StateFn(StateMachine::start);
    println!("{:?}", state_machine);

    state = state(&mut state_machine);
    println!("{:?}", state_machine);

    state = state(&mut state_machine);
    println!("{:?}", state_machine);

    // state(&mut state_machine);
    state.deref()(&mut state_machine);
    println!("{:?}", state_machine);
}
