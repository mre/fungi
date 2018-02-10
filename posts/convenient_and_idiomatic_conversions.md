# Convenient and idiomatic conversions in Rust

> source[00]
>
> 03 Aug 2016

## Key takeaways

The traits in `std::convert` provide a uniform API for converting values to
other types.

- `From<T>` and `Into<U>` are for conversions that cannot fail and consume the
  original value;
- `From<T>` for `U` converts a value of type `T` into one of type `U`;
- `Into<U>` for `T` inverts `From<T>` 's subject-object relationship;
- Implementing `From<T>` for `U` gives us an automatically derived `Into<U>` for
  `T` implementation;
- `TryFrom<T>` and `TryInto<U>` are the equivalent traits for conversions that
  may fail;
- `AsRef<T>` and `AsMut<T>` represent cheap reference-to-reference conversions,
  with some similarities to `Borrow<T>` and `BorrowMut<T>`;

## Introduction

We all convert data from one representation to another with some regularity.
There are several situations where this need pops up: converting a wide array of
types into a more convenient type, converting “foreign” error types to our
libraries' error types, and encoding and decoding network packets of our custom
protocols. The first situation is probably the most common. For instance, in
some cases a plain `Vec<T>` is a convenient representation, so there are readily
available ways to convert values of other types, such as `VecDeque<T>`,
`BinaryHeap<T>`, `&[T]`, and `&str`, into `Vec<T>`.

Naturally, there is more one way to convert types in Rust, each with advantages
and disadvantages. We could:

- build the target types ourselves with struct literals, but that's tedious,
  repetitive and exposes implementation details;
- create specialized constructors for each source type (e.g.:
  `new_from_vec_deque`, `new_from_binary_heap`, `new_from_slice`), but that's
  just as tedious and we might miss some cases anyway;
- write generic constructors that accept a certain trait, but that might still
  cover less cases than we need and require additional constructors;
- cast enum variants to integers and vice-versa, but those conversions may have
  unexpected results;

You get the idea: there are myriad ways of converting types, but many of them
suck. There has to be a better way!
In this article, we'll explore how to do it in a more idiomatic way — and if you
read the key takeaways you already know how. The traits in the `std::convert`
module — `From<T>`, `Into<U>`, `TryFrom<T>`, `TryInto<U>`, `AsRef<U>`, and
`AsMut<U>` — have this exact purpose. Those traits provide a uniform API for
type conversion, and we'll be exploring how we can leverage them to achieve a
consistent and ergonomic API. Once you know about them, you'll start noticing
them everywhere in the documentation. I hope that, by the end of this article
you'll probably appreciate them as much as I do.

## From and Into

`From<T>` represents the conversion of a value of type `T` into a target type
(`impl From<T> for TargetType`). This conversion may or may not be
computationally expensive, but we can usually assume it isn't cheap. Let's have
a look at its definition:

```rust
#[stable(feature = "rust1", since = "1.0.0")]
pub trait From<T>: Sized {
    /// Performs the conversion.
    #[stable(feature = "rust1", since = "1.0.0")]
    fn from(T) -> Self;
}
```

`From<T>` contains a single method signature, `from()`, which we'll have to
implement to perform the conversion. Inspecting `from()`'s signature, we can
tell that it _moves_ (or consumes) the argument. Its return value, `Self`, also
clues us in to the fact that the conversion may not fail. Later in this article,
we'll look into `TryFrom<T>` for conversions that may fail. `From<T>` is also a
reflexive trait, which means that conversion of a value into its own type
(`From<T> for T`) is implemented and returns the argument without modification.

Reading on, we arrive at the symmetrical companion trait of `From`, `Into<T>`.
Like `From`, `Into` has a short definition:

```rust
#[stable(feature = "rust1", since = "1.0.0")]
pub trait Into<T>: Sized {
    /// Performs the conversion.
    #[stable(feature = "rust1", since = "1.0.0")]
    fn into(self) -> T;
}
```

As we can see in the definition, `Into::into()` consumes `self` and returns `T`,
the opposite of `From::from()`, which consumes an argument `T` and returning
`Self`. Compare both ways of converting values:

```rust
// `from` can be called from either the `From` trait or the target type.
// Calling from the target type makes our intention clearer.

let converted_value = From::from(original_value);
let converted_value = TargetType::from(original_value);

// `into` is usually called directly on the original value, but we can
// also call it from the Into trait or the source type:

let converted_value = original_value.into();
let converted_value = Into::into(original_value);
```

While `From::from()` focuses on the target type, `Into::into()` focuses on the
original value; yet both express the same conversion. All the conversions above
are equivalent, choosing one of them is a matter of taste. Personally, I prefer
using `TargetType::from(value)` and `value.into()`. The former makes our
intention clearer, while the latter is shorter than `Into::into(value)`. Note
that we might need to add type annotations to disambiguate the intended target
type if we opt any form other than `TargetType::from()`, which clearly indicates
it.

A nice thing about implementing `From<T>` for `U` is that it implies `Into<U>`
for `T`, which means we get an automatic `Into` implementation for free (the
opposite isn't true):

```rust
// From implies Into
#[stable(feature = "rust1", since = "1.0.0")]
impl<T, U> Into<U> for T where U: From<T> {
    fn into(self) -> U {
        U::from(self)
    }
}
```

A secondary advantage of having an `Into` implementation (it doesn't matter if
it's explicit or automatic) is that we can use it to broaden function arguments
from a specific type to any type can be converted into the target type, as shown
in the following example:

```rust
// Instead of targetting a specific type like this:
fn do_something(value: TargetType<T>) {
    // ...
}

// We can broaden the accepted types with the following:
fn do_something<U: Into<TargetType<T>>>(value: U) {
    let converted_value = value.into();
    // ...
}
```

Alright, that's enough theory. A couple of examples will make it easier to
understand how all this works in practice.

## Example: SortedVec<T>

Suppose we have a sorted vector type, `SortedVec<T>`. Since it's a general data
structure, building a `SortedVec<T>` from slice-like and list-like types makes
sense, so we'll implement those conversions:

```rust
/// Our simple sorted vector structure is just a wrapper around a Vec
/// struct SortedVec<T>(Vec<T>);

/// Converting slices into SortedVec is pretty much expected.
impl<'a, T: Ord + Clone> From<&'a [T]> for SortedVec<T> {
    fn from(slice: &[T]) -> Self {
        let mut vec = slice.to_owned();
        vec.sort();
        SortedVec(vec)
    }
}

/// Converting a Vec is also expected.
/// We can sort the vector in place and then put it inside SortedVec.
impl<T: Ord + Clone> From<Vec<T>> for SortedVec<T> {
    fn from(mut vec: Vec<T>) -> Self {
        vec.sort();
        SortedVec(vec)
    }
}

/// Converting a LinkedList also makes sense, but it has no
/// slice representation, so we'll have to rely on its iterator.
impl<T: Ord + Clone> From<LinkedList<T>> for SortedVec<T> {
    fn from(list: LinkedList<T>) -> Self {
        let mut vec: Vec<T> = list.iter().cloned().collect();
        vec.sort();
        SortedVec(vec)
    }
}
```

Now, you might protest that the conversion from `Vec<T>` is redundant, because
we can get a slice from the vector and then convert the slice. That's absolutely
correct, dear reader. However, the implementation above avoids cloning the
vector, and, in my opinion, hiding any intermediate steps leads to a more
pleasant API.

As a result of the trait implementations above, we can call `SortedVec::from()`
without caring if the argument is a `slice`, `Vec` or `LinkedList`.

```rust
let vec = vec![1u8, 2, 3];

// Convert a slice
let sorted = SortedVec::from(&vec[1..]);

// ... a vector
let sorted = SortedVec::from(vec);

// ... a linked list
let mut linked_list: LinkedList<u8> = LinkedList::new();
linked_list.extend(&[1, 2, 3]);
let sorted = SortedVec::from(linked_list);
```

We can also go in the opposite direction and implement conversions from
`SortedVec<T>` into other types (for instance, into `Vec<T>`). However, there
are some restrictions about implementing traits for non-local, generic types —
check `error 0210` and the related `Rust RFC 1023`. As a rule of thumb, if the
non-local type isn't generic over some type parameter, you can implement `From`
for it.
