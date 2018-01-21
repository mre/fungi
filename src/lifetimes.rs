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
// By specifying the lifetime parameters in this function signature, we are not
// changing the lifetimes of any values passed in or returned, but we are saying
// that any values that do not adhere to this contract should be rejected by the
// borrow checker.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// is a straightforward example that should match your intuition from any
// language: string1 is valid until the end of the outer scope, string2 is valid
// until the end of the inner scope, and result references something that is
// valid until the end of the inner scope.
fn one() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}

// an example that will show that the lifetime of the reference in result must
// be the smaller lifetime of the two arguments.
fn two() {
    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {}", result);
    //
    // error: `string2` does not live long enough
    //    |
    // 6  |         result = longest(string1.as_str(), string2.as_str());
    //    |                                            ------- borrow occurs here
    // 7  |     }
    //    |     ^ `string2` dropped here while still borrowed
    // 8  |     println!("The longest string is {}", result);
    // 9  | }
    //    | - borrowed value needs to live until here
    //
    // Rust knows this because we annotated the lifetimes of the function
    // parameters and return values with the same lifetime parameter, 'a.
    //
    // When returning a reference from a function, the lifetime parameter for
    // the return type needs to match the lifetime parameter of one of the
    // arguments. If the reference returned does not refer to one of the
    // arguments, the only other possibility is that it refers to a value
    // created within this function, which would be a dangling reference since
    // the value will go out of scope at the end of the function.
}

// It is possible for structs to hold references, but we need to add a lifetime
// annotation on every reference in the struct’s definition.
fn three() {
    // A struct that holds a reference, so its definition needs a lifetime
    // annotation.
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

// Some lifetime patters are automatically derived by the compiler.
// fn first_word<'a>(s: &'a str) -> &'a str {
// The Rust team then programmed these patterns into the Rust compiler’s code so
// that the borrow checker can infer the lifetimes in these situations without
// forcing the programmer to explicitly add the annotations.
// The patterns programmed into Rust’s analysis of references are called the
// lifetime elision rules. These aren’t rules for programmers to follow; the
// rules are a set of particular cases that the compiler will consider, and if
// your code fits these cases, you don’t need to write the lifetimes explicitly.
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

pub fn sample() {
    one();
    two();
    three();
}
