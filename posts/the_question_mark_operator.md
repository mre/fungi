# Is the question mark operator ? equivalent to the try! macro?

https://stackoverflow.com/q/40545332

I am looking for a way to chain the usage of the `try!` macro in Rust and found
[his pull request that implements the ? operator](https://github.com/rust-lang/rust/pull/31954).

It seems like it has been merged, but I cannot seem to find any
documentation on it yet. Is it equivalent to the `try!` macro or are there
important differences?

Yes, the `?` operator is equivalent to try!(). `?` is now in stable Rust1.13, 
[released on November 10,2016](https://blog.rust-lang.org/2016/11/10/Rust-1.13.html).

The best source of documentation at the moment seems to be 
[RFC 0243](https://github.com/rust-lang/rfcs/blob/master/text/0243-trait-based-exception-handling.md). 

Note that the catch described in the RFC is not yet implemented at this writing 
([issue](https://github.com/rust-lang/rust/issues/31436)).

https://stackoverflow.com/a/40545445

# What is this question mark operator about?

I'm reading [the documentation for File](https://doc.rust-lang.org/std/fs/struct.File.html):

```
//..
let mut file = File::create("foo.txt")?;
//..
```

What is the `?` in this line?

https://stackoverflow.com/q/42917566

As you may have noticed, Rust does not have exceptions. It has panics, but their
functionality is limited (they cannot carry structured information) and their
use for error-handling is discouraged (they are meant for unrecoverable errors).

In Rust, error handling uses 
[Result](https://doc.rust-lang.org/std/result/enum.Result.html). A typical 
example would be:

```
fn halves_if_even(i: i32) -> Result<i32, Error> {
    if i % 2 == 0 { Ok(i/2) } else { Err(/* something */) }
}

fn do_the_thing(i: i32) -> Result<i32, Error> {
    let i = match halves_if_even(i) {
        Ok(i) => i,
        e => return e,
    };

    // use `i`
}
```

This is great because:

- when writing the code you cannot accidentally forget to deal with the error,
- when reading the code you can immediately see that there is a potential for
  error right here.
  
It's less than ideal, however, in that it is very verbose. This is where the
question mark operator `?` comes in.

The above can be rewritten as:

```
fn do_the_thing(i: i32) -> Result<i32, Error> {
    let i = halves_if_even(i)?;

    // use `i`
}
```

which is much more concise.

What `?` does here is equivalent to the `match` statement above. In short: 

> it unpacks the Result if OK and returns the error if not.

It's a bit magic, but error handling needs some magic to cut down the
boilerplate, and unlike exceptions it is immediately visible which function
calls may or may not error out: those that are adorned with `?`.

https://stackoverflow.com/a/42921174
