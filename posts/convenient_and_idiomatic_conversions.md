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

## Example: PacketType

Let's take a different example. Suppose we are now implementing a library for a
network protocol where the first byte in a packet header tells us the packet
type. A reasonable solution is representing the packet types with an
enumeration, where each variant maps to a packet type. For instance:

```rust
/// Represents a packet type.
/// Associated with each variant is its raw numeric representation.
enum PacketType {
    Data  = 0, // packet carries a data payload
    Fin   = 1, // signals the end of a connection
    State = 2, // signals acknowledgment of a packet
    Reset = 3, // forcibly terminates a connection
    Syn   = 4, // initiates a new connection with a peer
}
```

Given this representation, how shall we convert to and from the byte
representation? The traditional way, very common in C and C++ programs, is to
simply cast the values from one type to another. That can also be done in Rust;
for instance, converting `PacketType::Data` into a byte is as simple as
`PacketType::Data` as `u8`. That seems to take care of encoding a `PacketType`
into a byte representation, but we aren't done yet.

Did you notice that each `PacketType` variant has an associated value? They
define the variants' representation in the generated code. If we followed the
usual Rust style and didn't assign the variants any values, the numeric
representation of each variant would depend on the order they are declared,
which can lead to errors if we simply cast `enum` variants into numeric types. A
better way to convert the `enum` variants to the correct values is an explicit
match:

```rust
impl From<PacketType> for u8 {
    fn from(original: PacketType) -> u8 {
        match original {
            PacketType::Data  => 0,
            PacketType::Fin   => 1,
            PacketType::State => 2,
            PacketType::Reset => 3,
            PacketType::Syn   => 4,
        }
    }
}
```

Pretty straightforward, right? Since the mapping from `PacketType` to `u8` is
contained in the implementation of `From`, we can remove the values assigned to
`PacketType`'s variants, resulting in a cleaner enum definition.

### Behavior considered undefined

> Invalid values in primitive types, even in private fields/locals:
>
> A discriminant in an enum not included in the type definition

Although we can map any `PacketType` variant into an `u8` value, we can't do the
reverse and map any `u8` into a PacketType: there are too many `u8`s and not
enough `PacketTypes`!

So for the `u8` to `PacketType` conversion, we can't simply `match` on `u8`
value and return the appropriate `PacketType` variant like we did for the
opposite conversion. We need a way to signal that the conversion failed, but
calling `panic!()` is not an acceptable option. We need a fallible `From`.

## “Do or do not; there is no Try”

We saw that the conversions made by `From` and `Into` must not fail. However,
sometimes we deal with types that don't fully map onto one another, so we need
fallible versions of those traits. Fortunately, there's both `TryFrom` and
`TryInto`, which return a `Result<TargetType, ErrorType>`. Both live in
`std::convert` along with their infallible siblings, but their exact details and
implications are still under debate, which means they're still marked as
unstable. To use them, we can restrict ourselves to the nightly version of the
compiler, use the `try_from` crate, or paste their definitions somewhere in our
crates (they're really short).

Let's have a look at `TryFrom`'s definition (as of Rust 1.10.0):

```rust
#[unstable(feature = "try_from", issue = "33417")]
pub trait TryFrom<T>: Sized {
    /// The type returned in the event of a conversion error.
    type Err;

    /// Performs the conversion.
    fn try_from(T) -> Result<Self, Self::Err>;
}
```

First we have a stability attribute marking the trait as unstable, followed by
the trait definition itself. We can see it has an associated type, `Err`, for
the cases where the conversion fails. As expected, we have a `try_from` method
instead of `from`, which returns `Result<Self, Self::Err>` instead of `Self`.

Keeping with our example, we would have:

```rust
impl TryFrom<u8> for PacketType {
    type Err = ParseError;
    fn try_from(original: u8) -> Result<Self, Self::Err> {
        match original {
            0 => Ok(PacketType::Data),
            1 => Ok(PacketType::Fin),
            2 => Ok(PacketType::State),
            3 => Ok(PacketType::Reset),
            4 => Ok(PacketType::Syn),
            n => Err(ParseError::InvalidPacketType(n))
        }
    }
}
```

In this example, we return the corresponding `PacketType` variant for values
which can be mapped and an error for the remaining ones. This error type
preserves the original value, which is potentially useful for debugging
purposes, but we could just discard it instead.

## AsRef and AsMut

Last but not least, we're going to examine the remaining traits in the
`std::convert` module: `AsRef<T>` and `AsMut<T>`. Like the other traits in this
module, they are used to implement conversions among types. However, whereas the
other traits _consume values_ and may perform costly operations, `AsRef<T>` and
`AsMut<T>` are used to implement cheap, _reference-to-reference_ conversions.

As you have probably guessed from their names, `AsRef<T>` converts an immutable
reference to a value into another immutable reference, while `AsMut<T>` does the
same for mutable references.

Since they're both very similar, we're going to explore them at the same time.
Let's start with their definitions:

```rust
#[stable(feature = "rust1", since = "1.0.0")]
pub trait AsRef<T: ?Sized> {
    /// Performs the conversion.
    #[stable(feature = "rust1", since = "1.0.0")]
    fn as_ref(&self) -> &T;
}

#[stable(feature = "rust1", since = "1.0.0")]
pub trait AsMut<T: ?Sized> {
    /// Performs the conversion.
    #[stable(feature = "rust1", since = "1.0.0")]
    fn as_mut(&mut self) -> &mut T;
}
```

Both take references to self and return references to the target type with the
same mutability as `self`. Using these traits requires no more than calling
`as_ref()` or `as_mut()` on a value, depending on which conversion we need, like
so: `value.as_ref()`.

Implementing `AsRef<T>` and `AsMut<T>` is sensible and easy when the source type
is a wrapper around the target type, like the `SortedVec<T>` example we used
before. Since `SortedVec<T>` relies on a `Vec<T>`, implementing both traits is
painless:

```rust
/// SortedVec<T> is a tuple struct, containing a single Vec<T>.
struct SortedVec<T>(Vec<T>);

/// Implementing AsRef<Vec<T>> for SortedVec<T> only requires
/// returning a reference to SortedVec<T>'s single field.
impl<T> AsRef<Vec<T>> for SortedVec<T> {
    fn as_ref(&self) -> &Vec<T> {
        &self.0
    }
}

/// Implementing AsMut<Vec<T>> is just as easy.
/// Note that this allows the user to mutate the underlying Vec
/// such that it's no longer sorted, so you might want to avoid
/// implementing this trait!
impl<T> AsMut<Vec<T>> for SortedVec<T> {
    fn as_mut(&mut self) -> &mut Vec<T> {
        &mut self.0
    }
}
```

`AsRef<T>` and `AsMut<T>` also allow us to broaden the argument type from a
specific reference type to any type that can be cheaply converted to the target
reference type, just like `Into<T>`:

```rust
fn manipulate_vector<T, V: AsRef<Vec<T>>>(vec: V) -> Result<usize, ()> {
    // ...
}

// Now we can call `manipulate_vector` with a Vec<T> or anything that can
// be cheaply converted to Vec<T>, such as SortedVec<T>.
let sorted_vec = SortedVec::from(vec![1u8, 2, 3]);
match manipulate_vector(sorted_vec) {
    // ...
}
```

`AsRef<T>` and `AsMut<T>` are very similar to `Borrow<T>` and `BorrowMut<T>`,
but semantically different. The Rust Programming Language Book discusses those
differences in detail, but as a rule of thumb, we choose `AsRef<T>` and
`AsMut<T>` when we want to convert references or when writing generic code, and
`Borrow<T>` and `BorrowMut<T>` when we wish to disregard whether a value is
owned or borrowed (for instance, we might want a value to have the same hash
independently of it being owned or not).

There are a few interesting generic implementations for `AsRef<T>` and
`AsMut<T>`:

```rust
// As lifts over &
#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, T: ?Sized, U: ?Sized> AsRef<U> for &'a T where T: AsRef<U> {
    fn as_ref(&self) -> &U {
        <T as AsRef<U>>::as_ref(*self)
    }
}

// As lifts over &mut
#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, T: ?Sized, U: ?Sized> AsRef<U> for &'a mut T where T: AsRef<U> {
    fn as_ref(&self) -> &U {
        <T as AsRef<U>>::as_ref(*self)
    }
}

// AsMut lifts over &mut
#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, T: ?Sized, U: ?Sized> AsMut<U> for &'a mut T where T: AsMut<U> {
    fn as_mut(&mut self) -> &mut U {
        (*self).as_mut()
    }
}
```

Those generic implementations may look intimidating, but looks are deceiving.
Reading them slowly, we can see the traits are implemented for references to
types that implement `AsRef<U>` or `AsMut<U>` (`&'a T where T: AsRef<U>`,
`&'a mut T where T: AsRef<U>` and `&'a mut T where T: AsMut<U>`). We can also
see that every implementation dereferences the argument, which is a reference.

The result is rather useful: these trait implementations make references to
references (to references to references...) behave as if they were simple,
direct references. That is to say, they make multiple-level deep references such
as `&&&&vec` (in the case of the first implementation) and `&&&& mut vec` (in
the case of the second) equivalent to `&vec`, while the third implementation
makes `&mut &mut vec` equivalent to `&mut vec`. After those conversions, any
compatible conversions we explicitly implemented can be applied.

