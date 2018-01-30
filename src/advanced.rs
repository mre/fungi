// https://doc.rust-lang.org/stable/book/second-edition/ch19-01-unsafe-rust.html
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

fn sample() {
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
    // Note that the resulting split_at_mut function is safe: we didn’t have to
    // add the unsafe keyword in front of it, and we can call this function from
    // safe Rust. We’ve created a safe abstraction to the unsafe code by writing
    // an implementation of the function that uses unsafe code in a safe way by
    // only creating valid pointers from the data this function has access to.

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
    // language’s ABI.

    // The extern keyword is also used for creating an interface that allows
    // other languages to call Rust functions. Instead of an extern block, we
    // can add the extern keyword and specifying the ABI to use just before the
    // fn keyword. We also add the #[no_mangle] annotation to tell the Rust
    // compiler not to mangle the name of this function. The call_from_c
    // function in this example would be accessible from C code, once we’ve
    // compiled to a shared library and linked from C:

    #[no_mangle]
    pub extern "C" fn call_from_c() {
        println!("Just called a Rust function from C!");
    }
    // This usage of extern does not require unsafe.

    // Mutable Static Variables
    // Global variables are called static in Rust.
    static HELLO_WORLD: &str = "Hello, world!";

    println!("name is: {}", HELLO_WORLD);
    // static variables are similar to constants: their names are also in
    // SCREAMING_SNAKE_CASE by convention, and we must annotate the variable’s
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
    // it’s single threaded, but having multiple threads accessing COUNTER would
    // likely result in data races.

    // Implementing an Unsafe Trait
    unsafe trait Foo {
        // methods go here
    }

    unsafe impl Foo for i32 {
        // method implementations go here
    }

    // Like unsafe functions, methods in an unsafe trait have some invariant
    // that the compiler cannot verify. By using unsafe impl, we’re promising
    // that we’ll uphold these invariants.
}
