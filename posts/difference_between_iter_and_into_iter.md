# What is the difference between iter and into_iter?

[source (StackOverflow)](https://stackoverflow.com/questions/34733811/what-is-the-difference-between-iter-and-into-iter#34745885)

### The first question is: "What is into_iter?"

`into_iter` comes from the `IntoIterator` trait:

```rust
pub trait IntoIterator
where
    <Self::IntoIter as Iterator>::Item == Self::Item,
{
    type Item;
    type IntoIter: Iterator;
    fn into_iter(self) -> Self::IntoIter;
}
```

You implement this trait when you want to specify how a particular type is to be
_converted into an iterator_. Most notably, if a type implements `IntoIterator`
it can be used in a `for loop`.

For example, `Vec` implements `IntoIterator`... thrice!

```
impl<T> IntoIterator for Vec<T>
impl<'a, T> IntoIterator for &'a Vec<T>
impl<'a, T> IntoIterator for &'a mut Vec<T>
```

Each variant is slightly different.

This one consumes the `Vec` and its iterator yields _values_ (`T` directly):

```rust
impl<T> IntoIterator for Vec<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(mut self) -> IntoIter<T> { /* ... */ }
}
```

The other two take the vector _by reference_ (don't be fooled by the signature
of `into_iter(self)` because `self` is a reference in both cases) and their
iterators will produce references to the elements inside `Vec`.

This one yields _immutable_ references:

```rust
impl<'a, T> IntoIterator for &'a Vec<T> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;

    fn into_iter(self) -> slice::Iter<'a, T> { /* ... */ }
}
```

While this one yields mutable references:

```rust
impl<'a, T> IntoIterator for &'a mut Vec<T> {
    type Item = &'a mut T;
    type IntoIter = slice::IterMut<'a, T>;

    fn into_iter(self) -> slice::IterMut<'a, T> { /* ... */ }
}
```

### What is the difference between `iter` and `into_iter`?

into_iter is a generic method to obtain an iterator, whether this iterator yields values, immutable references or mutable references is context dependent and can sometimes be surprising.

iter and iter_mut are ad-hoc methods. This works around the context-dependent bit and, by convention, let you obtain an iterator which will yield references.

The author of the Rust by Example post illustrates the surprise coming from the dependence on the context (i.e., the type) on which into_iter is called, and is also compounding the problem by using the fact that:

IntoIterator is not implemented for [T; N], only for &[T; N] and &mut [T; N]
When a method is not implemented for a value, it is automatically searched for references to that value instead
which is very surprising for into_iter since all types (except [T; N]) implement it for all 3 variations (value and references). It's not possible for the array to implement an iterator that yields values because it cannot "shrink" to give up its items.

As to why arrays implement IntoIterator (in such a surprising fashion): it's to make it possible to iterate over references to them in for loops.

IntoIterator trait: https://doc.rust-lang.org/std/iter/trait.IntoIterator.html
yields values: https://doc.rust-lang.org/std/vec/struct.Vec.html#impl-IntoIterator
yields immutable references: https://doc.rust-lang.org/std/vec/struct.Vec.html#impl-IntoIterator-1
yields mutable references: https://doc.rust-lang.org/std/vec/struct.Vec.html#impl-IntoIterator-2

http://hermanradtke.com/2015/06/22/effectively-using-iterators-in-rust.html
