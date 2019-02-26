use std::cmp;
use std::cmp::Ordering;

#[allow(dead_code)]
fn unbox<T>(value: Box<T>) -> T {
    *value
}

#[derive(Debug)]
struct TNode<K, V> {
    left: Option<Box<TNode<K, V>>>,
    right: Option<Box<TNode<K, V>>>,
    key: K,
    value: V,
}

impl<K, V> TNode<K, V> {
    fn new(k: K, v: V) -> TNode<K, V> {
        return TNode {
            left: None,
            right: None,
            key: k,
            value: v,
        };
    }
}

impl<K: Ord, V> TNode<K, V> {
    fn max_depth(&self) -> usize {
        return match (&self.left, &self.right) {
            (None, None) => 0,
            (None, Some(r)) => 1 + r.max_depth(),
            (Some(l), None) => 1 + l.max_depth(),
            (Some(l), Some(r)) => 1 + cmp::max(l.max_depth(), r.max_depth()),
        };
    }

    fn min_depth(&self) -> usize {
        return match (&self.left, &self.right) {
            (None, None) => 0,
            (None, Some(r)) => 1 + r.max_depth(),
            (Some(l), None) => 1 + l.max_depth(),
            (Some(l), Some(r)) => 1 + cmp::min(l.max_depth(), r.max_depth()),
        };
    }

    fn insert(&mut self, n: TNode<K, V>) {
        match n.key.cmp(&self.key) {
            Ordering::Less => match self.left {
                None => self.left = Some(Box::new(n)),
                Some(ref mut l) => l.insert(n),
            },
            Ordering::Greater => match self.right {
                None => self.right = Some(Box::new(n)),
                Some(ref mut r) => r.insert(n),
            },
            Ordering::Equal => {}
        }
    }

    fn is_balanced(&self) -> bool {
        return (self.max_depth() - self.min_depth()) <= 1;
    }
}

fn main() {
    let mut root: TNode<usize, usize> = TNode {
        left: None,
        right: None,
        key: 0,
        value: 0,
    };
    println!("root key/value: {}/{}", root.key, root.value);
    root.left = Some(Box::new(TNode::new(1, 1)));
    root.right = Some(Box::new(TNode::new(2, 2)));
    root.insert(TNode::new(3, 3));
    root.insert(TNode::new(4, 4));
    println!("tree: {:?}", root);
    println!("max depth: {:?}", root.max_depth());
    println!("min depth: {:?}", root.min_depth());
    println!("is balanced? {:?}", root.is_balanced());
    // max depth: 3
    // min depth: 1
    // is balanced? false
}
