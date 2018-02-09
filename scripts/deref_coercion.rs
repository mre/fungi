// error[E0658]: type ascription is experimental (see issue #23416)
// https://github.com/rust-lang/rust/issues/23416
#![feature(type_ascription)]

#[derive(Debug)]
struct Foo {
    bar: u32,
}

impl Foo {
    fn number(&self) -> u32 {
        self.bar
    }

    fn method_ref_only(&self, r: &u32) -> u32 {
        self.bar + r
    }
}

fn ref_only(r: &u32) -> u32 {
    *r + 2
}

impl std::ops::Deref for Foo {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.bar
    }
}

// Compile and Run:
// rustc scripts/deref_coercion.rs --out-dir ./target/ && ./target/deref_coercion
//
fn main() {
    println!("// An experiment to prove the 'deref coercions'.");
    println!("// Let's start with a simple struct Foo.");
    let f = Foo { bar: 0 };
    println!("// Rust has derived Debug for us.");
    println!("Hello, Foo {:?}", f);
    println!("// Foo has a method 'number' to access it's internal field.");
    println!("// Can be used with the value");
    println!("Foo number is {} (using a value)", f.number());
    println!("// or can be used with a pointer");
    println!("Foo number is {}", &f.number());
    println!("// A method function can be desugared in its associated function");
    // https://doc.rust-lang.org/book/second-edition/ch05-03-method-syntax.html#associated-functions
    println!("Foo number (sugarfree) is {}", Foo::number(&f));
    let n = 10;
    let r = &n;
    println!("// A function that takes only references is easy to define");
    println!("Ref only please: {}", ref_only(r));
    println!("// and won't work with a value: error[E0308]: mismatched types");
    // println!("Ref only please: {}", ref_only(n));
    println!("// A method that takes only references is easy to define");
    println!("Ref only please: {}", f.method_ref_only(r));
    println!("// works also with associated function");
    println!("Ref only (sugarfree): {}", Foo::method_ref_only(&f, r));
    println!("// Since we defined the Deref Trait for the Foo struct");
    // we pass a reference to the struct Foo as parameter in the invocation of
    // the function ref_only and when this ref is dereferenced we get the u32
    // that is required by the function.
    println!("u32 references only, from a Foo: {}", ref_only(&f));

    struct StringContainer {
        f: String,
    };
    impl std::ops::Deref for StringContainer {
        type Target = String;
        fn deref(&self) -> &String {
            &self.f
        }
    }
    let sc = StringContainer {
        f: String::from("some string"),
    };
    println!(
        "Deref a StringContainer, length of the content: {}",
        String::len(&sc)
    );

    // Another example

    type StringFn = fn(u: u32) -> String;

    // https://doc.rust-lang.org/std/marker/trait.Copy.html
    // We can derive a `Copy` implementation. `Clone` is also required, as it's
    // a supertrait of `Copy`.
    //
    // But:
    // https://stackoverflow.com/questions/38215753/how-do-i-implement-clone-copy-for-an-enum-that-contains-a-string/38216027#38216027
    // Copy designates types for which making a bitwise copy creates a valid
    // instance without invalidating the original instance. This isn't true for
    // String, because String contains a pointer to the string data on the heap
    // and assumes it has unique ownership of that data, and thus when you drop
    // a String, it deallocates the data on the heap, and if you made a bitwise
    // copy of a String, then both instances would try to deallocate the same
    // memory block, which is undefined behaviour. Since String doesn't
    // implement Copy, your enum cannot implement Copy either, because the
    // compiler enforces that Copy types are composed only of Copy data members.
    // Clone merely provides a standard clone method, and it's up to each
    // implementor to decide how to implement it. String does implement Clone,
    // so you can put #[derive(Clone)] on your enum.
    //
    // So, instead of: #[derive(Debug, Copy, Clone)]
    // we will have:
    #[derive(Debug, Clone)]
    struct Bar {
        fou: u32,
        fos: String,
        foufn: StringFn,
    }

    impl<'b> std::ops::Deref for Bar {
        type Target = StringFn;
        // https://doc.rust-lang.org/book/first-edition/borrow-and-asref.html
        // https://doc.rust-lang.org/stable/book/second-edition/ch10-03-lifetime-syntax.html#thinking-in-terms-of-lifetimes
        // https://doc.rust-lang.org/nightly/book/second-edition/ch15-02-deref.html#implementing-the-deref-trait-defines-how-to-treat-a-type-like-a-reference
        //
        // Implementing the Deref Trait Defines How To Treat a Type Like a Reference
        // When we type *y what Rust actually ran behind the scenes was this
        // code:
        //   *(y.deref())
        // Rust substitutes the * operator with a call to the deref method and
        // then a plain dereference so that we don’t have to think about when we
        // have to call the deref method or not. This feature of Rust lets us
        // write code that functions identically whether we have a regular
        // reference or a type that implements Deref.
        //
        // The reason the deref method returns a reference to a value, and why
        // the plain dereference outside the parentheses in *(y.deref()) is
        // still necessary, is because of ownership. If the deref method
        // returned the value directly instead of a reference to the value, the
        // value would be moved out of self. We don’t want to take ownership of
        // the inner value inside MyBox<T> in this case and in most cases where
        // we use the dereference operator.
        //
        // Note that replacing * with a call to the deref method and then a call
        // to * happens once, each time we type a * in our code. The
        // substitution of * does not recurse infinitely.
        fn deref<'a>(&'a self) -> (&Self::Target) {
            println!("Dereferencing Bar as &StringFn");
            &(self.foufn)
        }
    }

    let mut b: Bar = Bar {
        fou: 10,
        fos: String::from("bar"),
        foufn: (|_x| String::from("foufn dereferenced and invoked")),
    };

    println!(
        "Bar is an struct with a couple of fields {} {} and...",
        b.fou, b.fos
    );

    println!("Dereferencing a function is possible: {}", (&b)(b.fou));

    b = Bar {
        fou: 10,
        fos: String::from("bar"),
        foufn: (|x| x.to_string()),
    };

    println!("Dereferencing a function is possible: {}", (&b)(b.fou));

    // Borrow
    // https://doc.rust-lang.org/std/borrow/trait.Borrow.html
    // A trait for borrowing data.
    // In general, there may be several ways to "borrow" a piece of data.
    // The typical ways of borrowing a type T are &T (a shared borrow) and
    // &mut T (a mutable borrow). But types like Vec<T> provide additional
    // kinds of borrows: the borrowed slices &[T] and &mut [T].
    //
    // When writing generic code, it is often desirable to abstract over all
    // ways of borrowing data from a given type. That is the role of the
    // Borrow trait: if T: Borrow<U>, then &U can be borrowed from &T. A
    // given type can be borrowed as multiple different types. In
    // particular, Vec<T>: Borrow<Vec<T>> and Vec<T>: Borrow<[T]>.
    //
    // AsRef
    // https://doc.rust-lang.org/std/convert/trait.AsRef.html
    // AsRef is to be used when wishing to convert to a reference of another
    // type. Borrow is more related to the notion of taking the reference. It is
    // useful when wishing to abstract over the type of reference (&T, &mut T)
    // or allow both the referenced and owned type to be treated in the same
    // manner.
    // The key difference between the two traits is the intention:
    // Use AsRef when goal is to simply convert into a reference;
    // Use Borrow when goal is related to writing code that is agnostic to the
    // type of borrow and if is reference or value;
    //
    // Into
    // https://doc.rust-lang.org/std/convert/trait.Into.html
    // pub trait Into<T> {
    //     fn into(self) -> T;
    // }

    impl Into<u32> for Bar {
        fn into(self) -> u32 {
            self.fou
        }
    }

    impl Into<String> for Bar {
        fn into(self) -> String {
            self.fou.to_string()
        }
    }

    println!("Convert a Type in another (owned) type is delegated to Trait std::convert::Into");
    let mut bb: Bar = b.clone();
    let converted: u32 = bb.into();
    println!("Expecting a u32 here: {}", converted);
    bb: Bar = b.clone();
    let converted: String = bb.into();
    println!("Expecting a string here: {}", converted);
    println!("And this conversion mechanism works explicitly");
    // but you need to be _very_ explicit:
    // error[E0619]: the type of this value must be known in this context
    //
    // While here we can try something "experimental":
    // error[E0658]: type ascription is experimental (see issue #23416)
    println!("lenght of a Bar: {}", (b.into(): String).len());
}
