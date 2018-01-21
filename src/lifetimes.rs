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

// An error about lifetime
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
// error[E0106]: missing lifetime specifier
//    |
// 1  | fn longest(x: &str, y: &str) -> &str {
//    |                                 ^ expected lifetime parameter
//    |
//     = help: this function's return type contains a borrowed value, but the
//             signature does not say whether it is borrowed from `x` or `y`
//
// Lifetime annotations don’t change how long any of the references involved
// live. In the same way that functions can accept any type when the signature
// specifies a generic type parameter, functions can accept references with any
// lifetime when the signature specifies a generic lifetime parameter. What
// lifetime annotations do is relate the lifetimes of multiple references to
// each other.
//
// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime

// The longest function definition that specifies all the references in the
// signature must have the same lifetime, 'a.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
pub fn sample() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
