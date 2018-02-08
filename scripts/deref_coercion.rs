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
    struct Bar {
        foo: u32,
    }
}
