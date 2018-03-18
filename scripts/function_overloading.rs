// http://casualhacks.net/blog/2018-03-10/exploring-function-overloading/

// The trait which defines the custom interface
// Notice how the subject and argument are 'swapped'
trait CustomFoo {
    fn custom_foo(self, this: &Foo);
}

// Explicit custom implementation for every supported type
impl CustomFoo for i32 {
    fn custom_foo(self, this: &Foo) {
        println!("Foo({}) i32: {}", this.0, self);
    }
}
impl CustomFoo for char {
    fn custom_foo(self, this: &Foo) {
        println!("Foo({}) char: {}", this.0, self);
    }
}
impl<'a, S: AsRef<str> + ?Sized> CustomFoo for &'a S {
    fn custom_foo(self, this: &Foo) {
        println!("Foo({}) str: {}", this.0, self.as_ref());
    }
}

// Silly example to have *something*
struct Foo(bool);

impl Foo {
    // Make the function generic over the argument
    // Straight up dispatch into the trait for the custom implementation
    fn foo<T: CustomFoo>(&self, arg: T) {
        arg.custom_foo(self);
    }
}

// --------

// OverloadedFoobar is a trait to wrap and take as parameter two generics.
trait OverloadedFoobar<T, U> {
    fn overloaded_foobar(&self, tee: T, yu: U);
}

// Foobar is an empty struct that we will "overload" using the
// OverloadedFoobar trait.
struct Foobar;

// Foobar static implementations.
impl Foobar {
    // foo is a function that takes two generics as parameter, but also imposes
    // that the "Self" is an implementation of the trait OverloadedFoobar
    fn foobar<T, U>(&self, tee: T, yu: U)
    where
        Self: OverloadedFoobar<T, U>,
    {
        self.overloaded_foobar(tee, yu)
    }
}

// Foobar's implementation for OverloadedFoo that "binds" the two generics T and U
// to i32 and f32.
impl OverloadedFoobar<i32, f32> for Foobar {
    fn overloaded_foobar(&self, tee: i32, yu: f32) {
        println!("foobar<i32, f32>(tee: {}, yu: {})", tee, yu);
    }
}

// Foo's implementation for char.
// https://github.com/rust-lang/rfcs/blob/master/text/0490-dst-syntax.md
impl<'a, S: AsRef<str> + ?Sized> OverloadedFoobar<&'a S, char> for Foobar {
    fn overloaded_foobar(&self, tee: &'a S, yu: char) {
        println!("foo<&str, char>(tee: {}, yu: {})", tee.as_ref(), yu);
    }
}

fn main() {
    Foo(false).foo(13);
    Foo(true).foo('😆');
    Foo(true).foo("baz");

    Foobar.foobar(42, 3.14159);
    Foobar.foobar("hello", '😄');
    // Foobar.foo('😏', 13); // the trait bound is not satisfied
}
