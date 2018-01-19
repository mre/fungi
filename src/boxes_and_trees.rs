// https://matthias-endler.de/2017/boxes-and-trees/
//   - https://www.reddit.com/r/rust/comments/6tkyz3/of_boxes_and_trees_smart_pointers_in_rust/
//   - https://rust-leipzig.github.io/architecture/2016/12/20/idiomatic-trees-in-rust/
//   - https://doc.rust-lang.org/book/second-edition/ch15-00-smart-pointers.html
//
// Implement a binary tree data structure in Rust. Each binary tree has a root
// value, a left, and a right subtree.
// Use pointers, an indirection like Box, Rc, or &. These are different "pointer
// types" in Rust.

// & is called a borrow in Rust speech. It's the most common of the three. It's
// a reference to some place in memory, but it does not own the data it points
// to. As such, the lifetime of the borrow depends on its owner. Therefore we
// would need to add lifetime parameters here. This can make it tedious to use.
#[allow(dead_code)]
pub struct TreeWithBorrows<'a> {
    root: i64,
    left: &'a TreeWithBorrows<'a>,
    right: &'a TreeWithBorrows<'a>,
}

// Box is a smart pointer with zero runtime overhead. It owns the data it points
// to. We call it smart because when it goes out of scope it will first drop the
// data it points to and then itself. No manual memory management required.
//
// https://doc.rust-lang.org/std/boxed/struct.Box.html
#[allow(dead_code)]
pub struct TreeBox {
    root: i64,
    left: Box<TreeBox>,
    right: Box<TreeBox>,
}

// Rc is another smart pointer. It's short for "reference-counting". It keeps
// track of the number of references to a data structure. As soon as the number
// of references is down to zero, it cleans up after itself. Choose Rc if you
// need to have multiple owners of the same data in one thread. For
// multithreading, there's also Arc (atomic reference count).
//
// https://doc.rust-lang.org/std/rc/struct.Rc.html
//
use std::rc::Rc;
#[allow(dead_code)]
pub struct TreeRc {
    root: i64,
    left: Rc<TreeRc>,
    right: Rc<TreeRc>,
}

// https://doc.rust-lang.org/std/default/index.html
// https://doc.rust-lang.org/std/default/trait.Default.html
//
// pub trait Default {
//     fn default() -> Self;
// }
// A trait for giving a type a useful default value.
// Sometimes, you want to fall back to some kind of default value, and don't
// particularly care what it is.
//
// Rust implements Default for various primitives types.
// If you want to override a particular option, but still retain the other
// defaults:
//
// fn main() {
//     let options = SomeOptions { foo: 42, ..Default::default() };
// }
#[derive(Debug, Default)]
pub struct TreeBoxOpt {
    root: i64,
    left: Option<Box<TreeBoxOpt>>,
    right: Option<Box<TreeBoxOpt>>,
}

#[allow(dead_code)]
impl TreeBoxOpt {
    fn new(root: i64) -> TreeBoxOpt {
        TreeBoxOpt {
            root: root,
            ..Default::default()
        }
    }

    fn left(mut self, leaf: TreeBoxOpt) -> Self {
        self.left = Some(Box::new(leaf));
        self
    }

    fn right(mut self, leaf: TreeBoxOpt) -> Self {
        self.right = Some(Box::new(leaf));
        self
    }
}

pub mod samples {
    use super::TreeBoxOpt;

    pub fn boxes() {
        let t: TreeBoxOpt = TreeBoxOpt {
            root: 15,
            left: Some(Box::new(TreeBoxOpt {
                root: 12,
                left: None,
                right: Some(Box::new(TreeBoxOpt {
                    root: 13,
                    left: None,
                    right: None,
                })),
            })),
            right: Some(Box::new(TreeBoxOpt {
                root: 22,
                left: Some(Box::new(TreeBoxOpt {
                    root: 18,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(TreeBoxOpt {
                    root: 100,
                    left: None,
                    right: None,
                })),
            })),
        };
        println!("TreeBoxOpt: {:?}", t);
    }

    pub fn left_right() {
        TreeBoxOpt::new(15)
            .left(TreeBoxOpt::new(12).right(TreeBoxOpt::new(13)))
            .right(
                TreeBoxOpt::new(22)
                    .left(TreeBoxOpt::new(18))
                    .right(TreeBoxOpt::new(100)),
            );
    }
}
