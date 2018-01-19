// https://matthias-endler.de/2017/boxes-and-trees/
//
// Implement a binary tree data structure in Rust. Each binary tree has a root
// value, a left, and a right subtree.
// Use pointers, an indirection like Box, Rc, or &. These are different "pointer
// types" in Rust.

// & is called a borrow in Rust speech. It's the most common of the three. It's
// a reference to some place in memory, but it does not own the data it points
// to. As such, the lifetime of the borrow depends on its owner. Therefore we
// would need to add lifetime parameters here. This can make it tedious to use.

struct TreeWithBorrows<'a> {
    root: i64,
    left: &'a TreeWithBorrows<'a>,
    right: &'a TreeWithBorrows<'a>,
}

// Box is a smart pointer with zero runtime overhead. It owns the data it points
// to. We call it smart because when it goes out of scope it will first drop the
// data it points to and then itself. No manual memory management required.
//
// https://doc.rust-lang.org/std/boxed/struct.Box.html
struct TreeBox {
    root: i64,
    left: Box<TreeBox>,
    right: Box<TreeBox>,
}
