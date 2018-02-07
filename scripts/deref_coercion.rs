#[derive(Debug)]
struct Foo {
    bar: u32,
}

impl Foo {
    fn number(&self) -> u32 {
        self.bar
    }

    fn method_ref_only(&self, r: &u32) -> u32 {
        *r + &self.bar
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
    println!("Ref only please: {}", f.method_ref_only(r));
    println!("Ref only please: {}", Foo::method_ref_only(&f, r));
}
