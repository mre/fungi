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
//
// {
//     let r;         // -------+-- 'a
//                    //        |
//     {              //        |
//         let x = 5; // -+-----+-- 'b
//         r = &x;    //  |     |
//     }              // -+     |
//                    //        |
//     println!("r: {}", r); // |
//                    //        |
//                    // -------+
// }
// Annotations of the lifetimes of r and x, named 'a and 'b respectively.
//
// We’ve annotated the lifetime of r with 'a and the lifetime of x with 'b. As
// you can see, the inner 'b block is much smaller than the outer 'a lifetime
// block. At compile time, Rust compares the size of the two lifetimes and sees
// that r has a lifetime of 'a, but that it refers to an object with a lifetime
// of 'b. The program is rejected because the lifetime 'b is shorter than the
// lifetime of 'a: the subject of the reference does not live as long as the
// reference.
//
// When there are no dangling references:
// {
//     let x = 5;            // -----+-- 'b
//     //      |
//     let r = &x;           // --+--+-- 'a
//     //   |  |
//     println!("r: {}", r); //   |  |
//     // --+  |
// }                         // -----+

pub fn sample() {}
