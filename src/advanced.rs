// https://doc.rust-lang.org/stable/book/second-edition/ch19-01-unsafe-rust.html
// https://doc.rust-lang.org/stable/book/second-edition/ch19-02-advanced-lifetimes.html
// https://doc.rust-lang.org/stable/book/second-edition/ch19-03-advanced-traits.html

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
    fn distance<N, E, G: GGraph<N, E>>(graph: &G, start: &N, end: &N) -> u32 {
        // ...snip...
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
    fn distance<G: AGraph>(graph: &G, start: &G::Node, end: &G::Node) -> u32 {
        // ...snip...
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
    fn distance<N, E>(graph: &GGraph<N, E>, start: &N, end: &N) -> u32 {
        // ...snip...
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
}
