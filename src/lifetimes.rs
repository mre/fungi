// https://doc.rust-lang.org/stable/book/second-edition/ch10-03-lifetime-syntax.html

// every reference in Rust has a lifetime, which is the scope for which that
// reference is valid.
// The main aim of lifetimes is to prevent dangling references, which will cause
// a program to reference data other than the data we’re intending to reference.
//
// `x` does not live enough:
// {
//     let r;
//
//     {
//         let x = 5;
//         r = &x;
//     }
//
//     println!("r: {}", r);
// }
//
// error: `x` does not live long enough
//    |
// 6  |         r = &x;
//    |              - borrow occurs here
// 7  |     }
//    |     ^ `x` dropped here while still borrowed
// ...
// 10 | }
//    | - borrowed value needs to live until here
//
//  If Rust allowed this code to work, r would be referencing memory that was
//  deallocated when x went out of scope, and anything we tried to do with r
//  wouldn’t work correctly.
// The part of the compiler called the borrow checker compares scopes to
// determine that all borrows are valid.
pub fn sample() {}
