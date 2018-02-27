// As you can see, you can't implement a trait you didn't write for a type you
// didn't write. This is part of what's known as "coherence" and exists to
// prevent really weird things like linking against a library suddenly causing
// unrelated parts of your program to change behaviour.

// Aliasing `Option` to `MyOption` doesn't work either because, as you say, it's
// an alias. That is, it's just another name for the same thing, it's not an
// actual, different type.

use std::fmt;

// Now, if you write a wrapper around `Option` like so:

#[allow(dead_code)]
struct MyOption<T>(Option<T>);

// then `MyOption` will be a new, distinct type that you can implement a trait
// for. Of course, you'll want to write methods to wrap and unwrap the actual
// Option you're storing.

// ... But this is all rather irrelevant since you could also just derive Debug
// for your struct and use that.

#[derive(Debug)]
struct MyStruct {
    foo: i32,
}

fn one() {
    let maybe_my_struct: Option<Box<MyStruct>> = Some(Box::new(MyStruct { foo: 42 }));
    println!("{:?}", maybe_my_struct);
}

// Or, if you really want the custom display logic for the Option<Box<MyStruct>>
// combination, you can use a marker value (this same approach is used by Path
// in the standard library, incidentally). Like so:

// This is the marker we'll use to define our custom Display impl.
struct MmsDisplay<'a>(&'a Option<Box<MyStruct>>);

// This trait lets us extend Option<Box<MyStruct>> with a new method.
trait CustomMmsDisplay {
    fn display<'a>(&'a self) -> MmsDisplay<'a>;
}

impl CustomMmsDisplay for Option<Box<MyStruct>> {
    fn display<'a>(&'a self) -> MmsDisplay<'a> {
        MmsDisplay(self)
    }
}

// And here's the display logic.
impl<'a> fmt::Display for MmsDisplay<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self.0 {
            Some(ref ms) => write!(formatter, "{}", ms.foo),
            None => write!(formatter, "No struct"),
        }
    }
}

fn two() {
    let maybe_my_struct: Option<Box<MyStruct>> = Some(Box::new(MyStruct { foo: 42 }));
    println!("{:?}", maybe_my_struct);

    // Instead of displaying directly, display via a custom marker.
    println!("{}", maybe_my_struct.display());
    println!("{}", None::<Box<MyStruct>>.display());
}

// Source: https://stackoverflow.com/a/30554247
//
// rustc scripts/print_option_struct.rs --out-dir ./target/ && ./target/print_option_struct
fn main() {
    one();
    two();
}
