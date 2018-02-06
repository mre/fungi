# Implicit Deref Coercions with Functions and Methods

https://doc.rust-lang.org/book/second-edition/ch15-02-deref.html#implicit-deref-coercions-with-functions-and-methods

_Deref coercion_ is a convenience that Rust performs on arguments to functions
and methods.

_Deref coercion_ converts a reference to a type that implements Deref into a
reference to a type that Deref can convert the original type into.

_Deref coercion_ happens automatically when we pass a reference to a value of a
particular type as an argument to a function or method that doesn't match the
type of the parameter in the function or method definition, and there's a
sequence of calls to the deref method that will convert the type we provided
into the type that the parameter needs.

_Deref coercion_ was added to Rust so that programmers writing function and
method calls don't need to add as many explicit references and dereferences with
`&` and `*`. This feature also lets us write more code that can work for either
references or smart pointers.

To illustrate deref coercion in action, let's use the `MyBox<T>` type we defined
in Listing 15-10 as well as the implementation of `Deref` that we added in
Listing 15-12. Listing 15-13 shows the definition of a function that has a
string slice parameter:


```rust
// Filename: src/main.rs

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
```

Listing 15-13: A `hello` function that has the parameter name of type `&str`

We can call the `hello` function with a string slice as an argument, like
`hello("Rust")`; for example. _Deref coercion_ makes it possible for us to call
hello with a reference to a value of type `MyBox<String>`, as shown in Listing
15-14:

```rust
// Filename: src/main.rs

fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}
```

Listing 15-14: Calling `hello` with a reference to a `MyBox<String>`, which
works because of _deref coercion_

Here we're calling the `hello` function with the argument `&m`, which is a
reference to a `ryBox<String>` value. Because we implemented the `Deref` trait
on `MyBox<T>` in Listing 15-12, Rust can turn `&MyBox<String>` into `&String` by
calling `deref`. The standard library provides an implementation of `Deref` on
`String` that returns a string slice, which we can see in the API documentation
for `Deref`.

Rust calls `deref` again to turn the `&String` into `&str`, which matches the
`hello` function's definition.

If Rust didn't implement _deref coercion_, in order to call `hello` with a value
of type `&MyBox<String>`, we'd have to write the code in Listing 15-15 instead
of the code in Listing 15-14:

```rust
Filename: src/main.rs

fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);
}
```

Listing 15-15: The code we'd have to write if Rust didn't have _deref coercion_

The `(*m)` is dereferencing the `MyBox<String>` into a `String`. Then the `&`
and `[..]` are taking a string slice of the `String` that is equal to the whole
string to match the signature of `hello`. The code without _deref coercions_ is
harder to read, write, and understand with all of these symbols involved.
_Deref coercion_ makes it so that Rust takes care of these conversions for us
automatically.

When the Deref trait is defined for the types involved, Rust will analyze the
types and use _Deref::deref_ as many times as it needs in order to get a
reference to match the parameter's type. This is resolved at compile time, so
there is no run-time penalty for taking advantage of deref coercion!
