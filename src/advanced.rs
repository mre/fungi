// https://doc.rust-lang.org/stable/book/second-edition/ch19-01-unsafe-rust.html
// https://doc.rust-lang.org/stable/book/second-edition/ch19-02-advanced-lifetimes.html
// https://doc.rust-lang.org/stable/book/second-edition/ch19-03-advanced-traits.html
// https://doc.rust-lang.org/stable/book/second-edition/ch19-04-advanced-types.html

// Unsafe Rust
// Dereferencing a Raw Pointer
// we can have an immutable raw pointer and a mutable raw pointer, written as
// *const T and *mut T, respectively. In the context of raw pointers,
// “immutable” means that the pointer can't be directly assigned to after being
// dereferenced.
// Raw pointers are different than references and smart pointers in a few ways.
// Raw pointers:
//
// - Are allowed to ignore the borrowing rules and have both immutable and a
//   mutable pointer or multiple mutable pointers to the same location
// - Aren't guaranteed to point to valid memory
// - Are allowed to be null
// - Don't implement any automatic clean-up

#[allow(dead_code)]
#[allow(unused_variables)]
fn one() {
    // Creating raw pointers from references
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // We've created raw pointers by using as to cast an immutable and a mutable
    // reference into their corresponding raw pointer types.

    let address = 0x012345usize;
    let r = address as *const i32;

    // Creating a raw pointer to an arbitrary memory address

    // there's no unsafe block. You can create raw pointers in safe code, but you
    // can't dereference raw pointers and read the data being pointed to.

    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    // Unsafe Functions

    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // This function can't be implemented using only safe Rust

    // For simplicity, we're implementing split_at_mut as a function rather than a
    // method, and only for slices of i32 values rather than for a generic type T:
    //
    // fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    //     let len = slice.len();
    //     assert!(mid <= len);
    //     (&mut slice[..mid], &mut slice[mid..])
    // }
    //
    // error[E0499]: cannot borrow `*slice` as mutable more than once at a time
    //   --> <anon>:6:11
    //   |
    // 5 |     (&mut slice[..mid],
    //   |           ----- first mutable borrow occurs here
    // 6 |      &mut slice[mid..])
    //   |           ^^^^^ second mutable borrow occurs here
    // 7 | }
    //   | - first borrow ends here
    // Rust's borrow checker can't understand that we're borrowing different parts
    // of the slice; it only knows that we're borrowing from the same slice twice.
    // Borrowing different parts of a slice is fundamentally okay; our two &mut
    // [i32] slices aren't overlapping.

    use std::slice;

    fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = slice.len();
        let ptr = slice.as_mut_ptr();

        assert!(mid <= len);

        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid),
            )
        }
    }

    // we can use the as_mut_ptr method to get access to the raw pointer of a
    // slice. In this case, since we have a mutable slice to i32 values,
    // as_mut_ptr returns a raw pointer with the type *mut i32, which we've
    // stored in the variable ptr.
    // the slice::from_raw_parts_mut function does the reverse from the
    // as_mut_ptr and len methods: it takes a raw pointer and a length and
    // creates a slice.
    // Because slices are checked, they're safe to use once we've created them.
    // The function slice::from_raw_parts_mut is an unsafe function because it
    // takes a raw pointer and trusts that this pointer is valid. The offset
    // method on raw pointers is also unsafe, since it trusts that the location
    // some offset after a raw pointer is also a valid pointer. We've put an
    // unsafe block around our calls to slice::from_raw_parts_mut and offset to
    // be allowed to call them.
    //
    // Note that the resulting split_at_mut function is safe: we didn't have to
    // add the unsafe keyword in front of it, and we can call this function from
    // safe Rust. We've created a safe abstraction to the unsafe code by writing
    // an implementation of the function that uses unsafe code in a safe way by
    // only creating valid pointers from the data this function has access to.
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn two() {
    // Extern
    // The keyword extern facilitates creating and using a Foreign Function
    // Interface (FFI).

    extern "C" {
        fn abs(input: i32) -> i32;
    }

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
    // "C" defines which application binary interface (ABI) the external
    // function uses. The ABI defines how to call the function at the assembly
    // level. The "C" ABI is the most common, and follows the C programming
    // language's ABI.

    // The extern keyword is also used for creating an interface that allows
    // other languages to call Rust functions. Instead of an extern block, we
    // can add the extern keyword and specifying the ABI to use just before the
    // fn keyword. We also add the #[no_mangle] annotation to tell the Rust
    // compiler not to mangle the name of this function. The call_from_c
    // function in this example would be accessible from C code, once we've
    // compiled to a shared library and linked from C:

    // #[no_mangle]
    pub extern "C" fn call_from_c() {
        println!("Just called a Rust function from C!");
    }
    // This usage of extern does not require unsafe.
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn three() {
    // Mutable Static Variables
    // Global variables are called static in Rust.
    static HELLO_WORLD: &str = "Hello, world!";

    println!("name is: {}", HELLO_WORLD);
    // static variables are similar to constants: their names are also in
    // SCREAMING_SNAKE_CASE by convention, and we must annotate the variable's
    // type, which is &'static str in this case. Only references with the
    // 'static lifetime may be stored in a static variable.

    // Accessing immutable static variables is safe. Values in a static variable
    // have a fixed address in memory, and using the value will always access
    // the same data. Constants, on the other hand, are allowed to duplicate
    // their data whenever they are used.

    // Another way in which static variables are different from constants is
    // that static variables can be mutable. Both accessing and modifying
    // mutable static variables is unsafe.

    static mut COUNTER: u32 = 0;

    fn add_to_count(inc: u32) {
        unsafe {
            COUNTER += inc;
        }
    }
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
    // Any time that we read or write from COUNTER has to be within an unsafe
    // block. This code compiles and prints COUNTER: 3 as we would expect since
    // it's single threaded, but having multiple threads accessing COUNTER would
    // likely result in data races.

    // Implementing an Unsafe Trait
    unsafe trait Foo {
        // methods go here
    }

    unsafe impl Foo for i32 {
        // method implementations go here
    }

    // Like unsafe functions, methods in an unsafe trait have some invariant
    // that the compiler cannot verify. By using unsafe impl, we're promising
    // that we'll uphold these invariants.
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn four() {
    // Advanced Lifetimes
    //
    // Lifetime subtyping

    // a Context struct that holds a string slice
    // struct Context<'a>(&'a str);

    // a Parser struct that holds a reference to a Context instance
    // struct Parser<'a> {
    //     context: &'a Context<'a>,
    // }

    // impl<'a> Parser<'a> {
    //     // a parse method that always returns an error referencing the string
    //     // slice. our parse function returns a Result<(), &str>. That is, we
    //     // don't do anything on success, and on failure we return the part of
    //     // the string slice that didn't parse correctly.
    //     //
    //     // without the elision rule:
    //     // fn parse<'a>(&'a self) -> Result<(), &'a str> {
    //     fn parse(&self) -> Result<(), &str> {
    //         Err(&self.context.0[1..])
    //     }
    // }
    // fn parse_context(context: Context) -> Result<(), &str> {
    //     Parser { context: &context }.parse()
    // }

    // Parser and context need to outlive the entire function and be valid
    // before the function starts as well as after it ends in order for all the
    // references in this code to always be valid. Both the Parser we're
    // creating and the context parameter go out of scope at the end of the
    // function, though (since parse_context takes ownership of context)
    //   fn parse(&self) -> Result<(), &str> {
    // the elision rules: if we annotate the lifetimes of the references, the
    // signature would be:
    //   fn parse<'a>(&'a self) -> Result<(), &'a str> {
    // That is, the "error part" of the return value of parse has a lifetime
    // that is tied to the Parser instance's lifetime (that of &self in the
    // parse method signature).
    // The problem is that the parse_context function returns the value returned
    // from parse, so the lifetime of the return value of parse_context is tied
    // to the lifetime of the Parser as well. But the Parser instance created in
    // the parse_context function won't live past the end of the function (it's
    // temporary), and the context will go out of scope at the end of the
    // function (parse_context takes ownership of it).
    // The parse_context function can't see that within the parse function, the
    // string slice returned will outlive both Context and Parser, and that the
    // reference parse_context returns refers to the string slice, not to
    // Context or Parser.

    // We need a way to tell Rust that the string slice in Context and the
    // reference to the Context in Parser have different lifetimes and that the
    // return value of parse_context is tied to the lifetime of the string slice
    // in Context.

    // We could try only giving Parser and Context different lifetime parameters

    struct Context<'s>(&'s str);

    // struct Parser<'c, 's> {
    //     context: &'c Context<'s>,
    // }

    impl<'c, 's> Parser<'c, 's> {
        fn parse(&self) -> Result<(), &'s str> {
            Err(&self.context.0[1..])
        }
    }

    fn parse_context(context: Context) -> Result<(), &str> {
        Parser { context: &context }.parse()
    }

    // We've annotated the lifetimes of the references, but used different
    // parameters depending on whether the reference goes with the string slice
    // or with Context. We've also added an annotation to the string slice part
    // of the return value of parse to indicate that it goes with the lifetime
    // of the string slice in Context.

    // error[E0491]: in type `&'c Context<'s>`, reference has a longer lifetime than the data it references
    //  --> src/main.rs:4:5
    //   |
    // 4 |     context: &'c Context<'s>,
    //   |     ^^^^^^^^^^^^^^^^^^^^^^^^
    //   |
    // note: the pointer is valid for the lifetime 'c as defined on the struct at 3:0
    //  --> src/main.rs:3:1
    //   |
    // 3 | / struct Parser<'c, 's> {
    // 4 | |     context: &'c Context<'s>,
    // 5 | | }
    //   | |_^
    // note: but the referenced data is only valid for the lifetime 's as defined on the struct at 3:0
    //  --> src/main.rs:3:1
    //   |
    // 3 | / struct Parser<'c, 's> {
    // 4 | |     context: &'c Context<'s>,
    // 5 | | }
    //   | |_^

    // Rust doesn't know of any relationship between 'c and 's. In order to be
    // valid, the referenced data in Context with lifetime 's needs to be
    // constrained to guarantee that it lives longer than the reference to
    // Context that has lifetime 'c. If 's is not longer than 'c, then the
    // reference to Context might not be valid.

    // Which gets us to the point of this section: Rust has a feature called
    // lifetime subtyping, which is a way to specify that one lifetime parameter
    // lives at least as long as another one. In the angle brackets where we
    // declare lifetime parameters, we can declare a lifetime 'a as usual, and
    // declare a lifetime 'b that lives at least as long as 'a by declaring 'b
    // with the syntax 'b: 'a.

    // In our definition of Parser, in order to say that 's (the lifetime of the
    // string slice) is guaranteed to live at least as long as 'c (the lifetime
    // of the reference to Context), we change the lifetime declarations to look
    // like this:

    struct Parser<'c, 's: 'c> {
        context: &'c Context<'s>,
    }

    // Now, the reference to Context in the Parser and the reference to the
    // string slice in the Context have different lifetimes, and we've ensured
    // that the lifetime of the string slice is longer than the reference to the
    // Context.
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn five() {
    // Lifetime Bounds
    // we discussed how to use trait bounds on generic types. We can also add
    // lifetime parameters as constraints on generic types, which are called
    // lifetime bounds

    // struct Ref<'a, T>(&'a T);
    // error[E0309]: the parameter type `T` may not live long enough
    //    --> <anon>:1:19
    //    |
    //  1 | struct Ref<'a, T>(&'a T);
    //    |                   ^^^^^^
    //    |
    //    = help: consider adding an explicit lifetime bound `T: 'a`...
    //    note: ...so that the reference type `&'a T` does not outlive the data it points at
    //    --> <anon>:1:19
    //    |
    //  1 | struct Ref<'a, T>(&'a T);
    //    |                   ^^^^^^
    // the T: 'a syntax specifies that T can be any type, but if it contains any
    // references, the references must live at least as long as 'a:
    struct Ref<'a, T: 'a>(&'a T);

    // struct StaticRef<T: 'static>(&'static T);
    // Adding a 'static lifetime bound to T to constrain T to types that have
    // only 'static references or no references

    // Types without any references count as T: 'static. Because 'static means
    // the reference must live as long as the entire program, a type that
    // contains no references meets the criteria of all references living as
    // long as the entire program (since there are no references). Think of it
    // this way: if the borrow checker is concerned about references living long
    // enough, then there's no real distinction between a type that has no
    // references and a type that has references that live forever;
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn six() {
    // Trait object lifetimes
    // we learned about trait objects that consist of putting a trait behind a
    // reference in order to use dynamic dispatch. However, we didn't discuss
    // what happens if the type implementing the trait used in the trait object
    // has a lifetime.
    // Consider where we have a trait Foo and a struct Bar that holds a
    // reference (and thus has a lifetime parameter) that implements trait Foo,
    // and we want to use an instance of Bar as the trait object Box<Foo>:

    trait Foo {}

    struct Bar<'a> {
        x: &'a i32,
    }

    impl<'a> Foo for Bar<'a> {}

    let num = 5;
    let obj = Box::new(Bar { x: &num }) as Box<Foo>;

    // This code compiles without any errors, even though we haven't said
    // anything about the lifetimes involved in obj. This works because there
    // are rules having to do with lifetimes and trait objects:

    // - The default lifetime of a trait object is 'static.
    // - If we have &'a X or &'a mut X, then the default is 'a.
    // - If we have a single T: 'a clause, then the default is 'a.
    // - If we have multiple T: 'a-like clauses, then there is no default; we must be explicit.
    // When we must be explicit, we can add a lifetime bound on a trait object
    // like Box<Foo> with the syntax Box<Foo + 'a> or Box<Foo + 'static>,
    // depending on what's needed. Just as with the other bounds, this means
    // that any implementor of the Foo trait that has any references inside must
    // have the lifetime specified in the trait object bounds as those
    // references.
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn seven() {
    // Associated Types
    // Associated types are a way of associating a type placeholder with a
    // trait such that the trait method definitions can use these
    // placeholder types in their signatures. The implementor of a trait
    // will specify the concrete type to be used in this type's place for
    // the particular implementation.

    // An example of a trait with an associated type is the Iterator trait
    // provided by the standard library. It has an associated type named Item
    // that stands in for the type of the values that we're iterating over.
    pub trait Iterator {
        type Item;
        fn next(&mut self) -> Option<Self::Item>;
    }
    // Item is a placeholder type, and the return value of the next method will
    // return values of type Option<Self::Item>.
    // Implementors of this trait will specify the concrete type for Item, and
    // the next method will return an Option containing a value of whatever type
    // the implementor has specified.

    // impl Iterator for Counter {
    //     type Item = u32;
    //     fn next(&mut self) -> Option<Self::Item> {

    // pub trait Iterator<T> {
    //     fn next(&mut self) -> Option<T>;
    // }
    // we could also implement Iterator<String> for Counter, or any other type
    // as well, so that we'd have multiple implementations of Iterator for
    // Counter. In other words, when a trait has a generic parameter, we can
    // implement that trait for a type multiple times, changing the generic type
    // parameters' concrete types each time.

    // With associated types, we can't implement a trait on a type multiple
    // times. we can only choose once what the type of Item will be, since there
    // can only be one impl Iterator for Counter. We don't have to specify that
    // we want an iterator of u32 values everywhere that we call next on
    // Counter.

    trait GGraph<Node, Edge> {
        // methods would go here
    }

    trait AGraph {
        type Node;
        type Edge;

        // methods would go here
    }

    // Two graph trait definitions, GGraph using generics and AGraph using
    // associated types for Node and Edge.
    // With the GGraph trait defined using generics, our distance function
    // signature would have to look like:
    fn distance_a<N, E, G: GGraph<N, E>>(graph: &G, start: &N, end: &N) -> u32 {
        // ...snip...
        0
    }
    // The signature of a distance function that uses the trait GGraph and has
    // to specify all the generic parameters.
    // Our function would need to specify the generic type parameters N, E, and
    // G, where G is bound by the trait GGraph that has type N as its Node type
    // and type E as its Edge type. Even though distance doesn't need to know
    // the types of the edges, we're forced to declare an E parameter, because
    // we need to to use the GGraph trait and that requires specifying the type
    // for Edge.

    // the definition of distance that uses the AGraph trait with associated
    // types:
    fn distance_b<G: AGraph>(graph: &G, start: &G::Node, end: &G::Node) -> u32 {
        // ...snip...
        0
    }
    // The signature of a distance function that uses the trait AGraph and the
    // associated type Node This is much cleaner. We only need to have one
    // generic type parameter, G, with the trait bound AGraph. Since distance
    // doesn't use the Edge type at all, it doesn't need to be specified
    // anywhere. To use the Node type associated with AGraph, we can specify
    // G::Node.

    // Trait Objects with Associated Types

    // You may have been wondering why we didn't use a trait object in the
    // distance functions.
    // The signature for the distance function using the generic GGraph trait
    // does get a bit more concise using a trait object:
    fn distance_c<N, E>(graph: &GGraph<N, E>, start: &N, end: &N) -> u32 {
        // ...snip...
        0
    }
    // Specifying the Edge type is still required, though.
    // It is possible in general to use trait objects of traits that have
    // associated types, though; unction named traverse that doesn't need to use
    // the trait's associated types in other arguments. We do, however, have to
    // specify the concrete types for the associated types in this case. Here,
    // we've chosen to accept types that implement the AGraph trait with the
    // concrete type of usize as their Node type and a tuple of two usize values
    // for their Edge type:
    fn traverse(graph: &AGraph<Node = usize, Edge = (usize, usize)>) {
        // ...snip...
    }
}

fn eight() {
    // Operator Overloading and Default Type Parameters
    // The <PlaceholderType=ConcreteType> syntax is used in another way as well:
    // to specify the default type for a generic type. A great example of a
    // situation where this is useful is operator overloading.
    //

    // how to overload the + operator by implementing the Add trait on a Point
    // struct so that we can add two Point instances together:

    use std::ops::Add;

    #[derive(Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Add for Point {
        type Output = Point;

        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    // the Add trait in a bit more detail. Here's its definition:
    // trait Add<RHS=Self> {
    //     type Output;
    //     fn add(self, rhs: RHS) -> Self::Output;
    // }
    // it's a trait with one method and an associated type. The new part is the
    // RHS=Self in the angle brackets: this syntax is called default type
    // parameters. RHS is a generic type parameter (short for “right hand side”)
    // that's used for the type of the rhs parameter in the add method. If we
    // don't specify a concrete type for RHS when we implement the Add trait,
    // the type of RHS will default to the type of Self (the type that we're
    // implementing Add on).

    // use std::ops::Add;
    struct Millimeters(u32);
    struct Meters(u32);

    impl Add for Millimeters {
        type Output = Millimeters;

        fn add(self, other: Millimeters) -> Millimeters {
            Millimeters(self.0 + other.0)
        }
    }

    impl Add<Meters> for Millimeters {
        type Output = Millimeters;

        fn add(self, other: Meters) -> Millimeters {
            Millimeters(self.0 + (other.0 * 1000))
        }
    }

    // If we're adding Millimeters to other Millimeters, we don't need to
    // parameterize the RHS type for Add since the default Self type is what we
    // want. If we want to implement adding Millimeters and Meters, then we need
    // to say impl Add<Meters> to set the value of the RHS type parameter.
    // Default type parameters are used in two main ways:
    // - To extend a type without breaking existing code.
    // - To allow customization in a way most users don't want.
    // The Add trait is an example of the second purpose: most of the time,
    // you're adding two like types together. Using a default type parameter in
    // the Add trait definition makes it easier to implement the trait since you
    // don't have to specify the extra parameter most of the time.
}

fn nine() {
    // Fully Qualified Syntax for Disambiguation

    trait Pilot {
        fn fly(&self);
    }

    trait Wizard {
        fn fly(&self);
    }

    struct Human;

    impl Pilot for Human {
        fn fly(&self) {
            println!("This is your captain speaking.");
        }
    }

    impl Wizard for Human {
        fn fly(&self) {
            println!("Up!");
        }
    }

    impl Human {
        fn fly(&self) {
            println!("*waving arms furiously*");
        }
    }

    // When we call fly on an instance of Human, the compiler defaults to
    // calling the method that is directly implemented on the type,
    // In order to call the fly methods from either the Pilot trait or the
    // Wizard trait, we need to use more explicit syntax in order to specify
    // which fly method we mean.

    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    // However, associated functions that are part of traits don't have a self
    // parameter. When two types in the same scope implement that trait, Rust
    // can't figure out which type we mean unless we use fully qualified syntax.

    // Animal has the associated function baby_name, the implementation of
    // Animal for the struct Dog, and the associated function baby_name defined
    // on Dog directly.
    trait Animal {
        fn baby_name() -> String;
    }

    struct Dog;

    impl Dog {
        fn baby_name() -> String {
            String::from("Spot")
        }
    }

    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("puppy")
        }
    }

    println!("A baby dog is called a {}", Dog::baby_name());
    // Attempting to call the baby_name function from the Animal trait:
    // println!("A baby dog is called a {}", Animal::baby_name());
    // error[E0283]: type annotations required: cannot resolve `_: Animal`
    // Because Animal::baby_name is an associated function rather than a method,
    // and thus doesn't have a self parameter, Rust has no way to figure out
    // which implementation of Animal::baby_name we want.
    // we want to use the implementation of Animal for Dog, we need to use fully
    // qualified syntax, which is the most specific we can be when calling a
    // function.
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
    // We're providing Rust with a type annotation within the angle brackets,
    // and we're specifying that we want to call the baby_name method from the
    // Animal trait as implemented on Dog by saying that we want to treat the
    // Dog type as an Animal for this function call.
    // <Type as Trait>::function(receiver_if_method, next_arg, ...);
    // We only need to use this more verbose syntax in cases where there are
    // multiple implementations that use the same name and Rust needs help in
    // order to know which implementation we want to call.
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn ten() {
    // Supertraits to use one trait's functionality within another trait
    // Sometimes, we may want a trait to be able to rely on another trait also
    // being implemented wherever our trait is implemented, so that our trait
    // can use the other trait's functionality. The required trait is a
    // supertrait of the trait we're implementing.
    // In the implementation of outline_print, since we want to be able to use
    // the Display trait's functionality, we need to be able to say that the
    // OutlinePrint trait will only work for types that also implement Display
    // and provide the functionality that OutlinePrint needs.
    // in the trait definition by specifying OutlinePrint: Display. It's like
    // adding a trait bound to the trait.

    use std::fmt;

    trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }
    // Because we've specified that OutlinePrint requires the Display trait, we
    // can use to_string in outline_print (to_string is automatically
    // implemented for any type that implements Display).
    // If we try to implement OutlinePrint on a type that doesn't implement Display, such as the Point struct:
    //
    struct Point {
        x: i32,
        y: i32,
    }

    // impl OutlinePrint for Point {}
    // error[E0277]: the trait bound `Point: std::fmt::Display` is not satisfied

    // Once we implement Display on Point and satisfy the constraint that
    // OutlinePrint requires, like so:

    // use std::fmt;
    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    impl OutlinePrint for Point {}

    // then implementing the OutlinePrint trait on Point will compile
    // successfully and we can call outline_print on a Point instance.
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn eleven() {
    // The Newtype Pattern to Implement External Traits on External Types

    // we mentioned the orphan rule: we're allowed to implement a trait on a
    // type as long as either the trait or the type are local to our crate. One
    // way to get around this restriction is to use the newtype pattern, which
    // involves creating a new type using a tuple struct with one field as a
    // thin wrapper around the type we want to implement a trait for. Then the
    // wrapper type is local to our crate, and we can implement the trait on the
    // wrapper. There's no runtime performance penalty for using this pattern.
    // The wrapper type is elided at compile time.

    // For example, if we wanted to implement Display on Vec, we can make a
    // Wrapper struct that holds an instance of Vec. Then we can implement
    // Display on Wrapper and use the Vec value:

    use std::fmt;

    struct Wrapper(Vec<String>);

    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Using ".0" because Wrapper is a Struct Tuple:
            // https://doc.rust-lang.org/stable/book/second-edition/ch03-02-data-types.html#grouping-values-into-tuples
            // Values can be extracted from the tuple using tuple indexing
            write!(f, "[{}]", self.0.join(", "))
        }
    }

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);

    //  The implementation of Display uses self.0 to access the inner Vec, and
    //  then we can use the functionality of the Display type on Wrapper.

    // The downside is that since Wrapper is a new type, it doesn't have the
    // methods of the value it's holding; we'd have to implement all the methods
    // of Vec like push, pop, and all the rest directly on Wrapper to delegate
    // to self.0 in order to be able to treat Wrapper exactly like a Vec.
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn twelve() {
    // Advanced Types
    // Using the Newtype Pattern for Type Safety and Abstraction

    // The newtype pattern where we create a new type as a tuple struct
    // with one field that wraps a type can also be useful for statically
    // enforcing that values are never confused, and is often used to indicate
    // the units of a value.

    // Another reason to use the newtype pattern is to abstract away some
    // implementation details of a type: the wrapper type can expose a different
    // public API than the private inner type would if we used it directly in
    // order to restrict the functionality that is available, for example. New
    // types can also hide internal generic types. F

    // Type Aliases Create Type Synonyms
    // The newtype pattern involves creating a new struct to be a new, separate
    // type. Rust also provides the ability to declare a type alias with the
    // type keyword to give an existing type another name.

    type Kilometers = i32;

    // This means Kilometers is a synonym for i32; Kilometers is not a separate,
    // new type. Values that have the type Kilometers will be treated exactly
    // the same as values of type i32:

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    // Since Kilometers is an alias for i32, they're the same type.
    // The main use case for type synonyms is to reduce repetition.
    //
    // Box<Fn() + Send + 'static>
    //
    // Writing this out in function signatures and as type annotations all over
    // the place can be tiresome and error-prone.

    // let f: Box<Fn() + Send + 'static> = Box::new(|| println!("hi"));
    // fn takes_long_type(f: Box<Fn() + Send + 'static>) {
    //     // ...snip...
    // }
    // fn returns_long_type() -> Box<Fn() + Send + 'static> {
    //     // ...snip...
    // }

    // A type alias makes this code more manageable by reducing the amount of
    // repetition this project has. Here, we've introduced an alias named Thunk
    // for the verbose type, and we can replace all uses of the type with the
    // shorter Thunk

    // type Thunk = Box<Fn() + Send + 'static>;
    // let f: Thunk = Box::new(|| println!("hi"));
    // fn takes_long_type(f: Thunk) {
    //     // ...snip...
    // }
    // fn returns_long_type() -> Thunk {
    //     // ...snip...
    // }

    // Another common use of type aliases is with the Result<T, E> type. Consider
    // the std::io module in the standard library. I/O operations often return a
    // Result<T, E>, since their operations may fail to work. There's a
    // std::io::Error struct that represents all of the possible I/O errors. Many of
    // the functions in std::io will be returning Result<T, E> where the E is
    // std::io::Error, such as these functions in the Write trait:

    // use std::io::Error;
    // use std::fmt;
    //
    // pub trait Write {
    //     fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    //     fn flush(&mut self) -> Result<(), Error>;
    //
    //     fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
    //     fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
    // }
    // We're writing Result<..., Error> a lot. As such, std::io has this type alias declaration:
    //
    // type Result<T> = Result<T, std::io::Error>;
    //
    // Because this is in the std::io module, the fully qualified alias that we can
    // use is std::io::Result<T>; that is, a Result<T, E> with the E filled in as
    // std::io::Error. The Write trait function signatures end up looking like this:
    //
    // pub trait Write {
    //     fn write(&mut self, buf: &[u8]) -> Result<usize>;
    //     fn flush(&mut self) -> Result<()>;
    //
    //     fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    //     fn write_fmt(&mut self, fmt: Arguments) -> Result<()>;
    // }

    // The Never Type, !, that Never Returns
    // Rust has a special type named !. In type theory lingo, it's called the
    // empty type, because it has no values. We prefer to call it the never
    // type. The name describes what it does: it stands in the place of the
    // return type when a function will never return.

    // fn bar() -> ! {
    //     // ...snip...
    // }

    // This is read as "the function bar returns never," and functions that
    // return never are called diverging functions. We can't create values of
    // the type !, so bar can never possibly return. What use is a type you can
    // never create values for?
    //
    // let guess: u32 = match guess.trim().parse() {
    //     Ok(num) => num,
    //     Err(_) => continue,
    // };

    // A match with an arm that ends in continue but that match arms must return
    // the same type. This doesn't work:
    // let guess = match guess.trim().parse()  {
    //     Ok(_) => 5,
    //     Err(_) => "hello",
    // }

    // What would the type of guess be here? It'd have to be both an integer and
    // a string, and Rust requires that guess can only have one type. So what
    // does continue return? Why are we allowed to return a u32 from one arm
    // and have another arm that ends with continue?
    // As you may have guessed, continue has a value of !. That is, when Rust
    // goes to compute the type of guess, it looks at both of the match arms.
    // The former has a value of u32, and the latter has a value of !. Since !
    // can never have a value, Rust is okay with this, and decides that the type
    // of guess is u32. The formal way of describing this behavior of ! is that
    // the never type unifies with all other types. We're allowed to end this
    // match arm with continue because continue doesn't actually return a value;
    // it instead moves control back to the top of the loop, so in the Err case,
    // we never actually assign a value to guess.

    // Another use of the never type is panic!. Remember the unwrap function
    // that we call on Option<T> values to produce a value or panic? Here's its
    // definition:
    // impl<T> Option<T> {
    //     pub fn unwrap(self) -> T {
    //         match self {
    //             Some(val) => val,
    //             None => panic!("called `Option::unwrap()` on a `None` value"),
    //         }
    //     }
    // }

    // Here, we know that val has the type T, and panic! has the type !, so the
    // result of the overall match expression is T. This works because panic!
    // doesn't produce a value; it ends the program. In the None case, we won't
    // be returning a value from unwrap, so this code is valid.

    // One final expression that has the type ! is a loop:
    // print!("forever ");
    // loop {
    //     print!("and ever ");
    // }
    // Here, the loop never ends, so the value of the expression is !. This
    // wouldn't be true if we included a break, however, as the loop would
    // terminate when it gets to the break.
}

#[allow(dead_code)]
#[allow(unused_variables)]
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
    eleven();
    twelve();
}
