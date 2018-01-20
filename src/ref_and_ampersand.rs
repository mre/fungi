// https://github.com/rust-lang/rust-by-example/issues/390

fn one() {
    let y = 'y';

    // `ref` on the left side of an assignment is like adding `&` on the right side
    let ref x1 = y;
    let x2 = &y;

    println!("{}", x1 == x2);
}

fn two() {
    struct Foo(i32);
    let foo = &Foo(42);
    // match is a "destructuring" (or destructive) operation; anything we apply
    // match to will be moved into the block by default:
    //
    // let maybe_name = Some(String::from("Alice"));
    // ...
    // match maybe_name {
    //     Some(n) => println!("Hello, {}", n),
    //     _ => {},
    // }
    // do_something_with(maybe_name)
    //
    // error: use of partially moved value: `maybe_name` [E0382]
    // note:  `(maybe_name:core::option::Option::Some).0` moved here because it
    //        has type `collections::string::String`, which is moved by default
    //        Some(n) => println!("Hello, {}", n),
    // help:  if you would like to borrow the value instead, use a `ref` binding as shown:
    //        Some(ref n) => println!("Hello, {}", n),
    //
    // This means that if we use `Some(ref n)` as pattern in the match branch
    // we will borrow `n` instead of moving it into the `match`.
    // `ref` is not something used in the match.
    //
    // - by default, variable are moved into the match branch;
    //   in `Some(n) => ...` the variable `n` is a `String`
    // - with ref, variables are borrowed and represented as references;
    //   in `Some(ref n) => ...` the variable `n` is a `&String`
    //
    // ref annotates pattern bindings to make them borrow rather than move. It
    // is not a part of the pattern as far as matching is concerned.
    match foo {
        // matching a reference to a Foo.
        // The syntax used in a pattern that destructures an object is analogous
        // to one used by the expression which created it.
        &Foo(n) => println!("Match: number {}", n),
    }
    // The `&` and `ref` differs in the pattern that they match. `&` is part of
    // the patter and discriminate the result: &String is matched; `ref`
    // instead match `Name(ref foo)` as `Name(foo)`
}

pub fn sample() {
    one();
    two();
}
