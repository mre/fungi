# Convenient and idiomatic conversions in Rust

> source[00]
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
