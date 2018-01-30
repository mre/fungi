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
// Rust’s borrow checker can’t understand that we’re borrowing different parts
// of the slice; it only knows that we’re borrowing from the same slice twice.
// Borrowing different parts of a slice is fundamentally okay; our two &mut
// [i32] slices aren’t overlapping.

use std::slice;

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (slice::from_raw_parts_mut(ptr, mid),
         slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
    }
}
