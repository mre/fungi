use std::collections::VecDeque;

// permutations without repetition:
// where n is the number of objects ((0..10).len())
// and r is the number of positions (chars.len())
// the total permutations are P(n,r) = n!/(n-r)!
//
// https://en.wikipedia.org/wiki/Permutation
// https://en.wikipedia.org/wiki/Heap's_algorithm
// Permutations by interchanges. B.R.Heap, The Computer Journal, 6(3) (1963)
// http://comjnl.oxfordjournals.org/content/6/3/293.full.pdf

fn permute<T, F: FnMut(&[T])>(used: &mut Vec<T>, unused: &mut VecDeque<T>, action: &mut F) {
    if unused.is_empty() {
        action(used);
    } else {
        for _ in 0..unused.len() {
            used.push(unused.pop_front().unwrap());
            permute(used, unused, action);
            unused.push_back(used.pop().unwrap());
        }
    }
}

fn main() {
    let mut queue = (1..4).collect::<VecDeque<_>>();
    let mut mtrix: Vec<Vec<i32>> = Vec::new();

    // a closure with trait Fn.
    // permute(&mut Vec::new(), &mut queue, &|perm| println!("{:?}", perm));
    //
    // http://doc.rust-lang.org/std/ops/trait.FnMut.html
    // https://stackoverflow.com/a/37949326
    // https://stackoverflow.com/a/30232500
    // http://huonw.github.io/blog/2015/05/finding-closure-in-rust/
    //
    // There are three traits for closures, all of which provide the
    // ...(...) call syntax:
    //
    // &self is Fn        | fn by_value(_: T) {}
    // &mut self is FnMut | fn by_mut(_: &mut T) {}
    // self is FnOnce     | fn by_ref(_: &T) {}

    fn act_push(m: &mut Vec<Vec<i32>>, v: Vec<i32>) {
        m.push(v)
    }

    permute(&mut Vec::new(), &mut queue, &mut |perm| {
        mtrix.push(perm.to_vec())
    });

    for (i, v) in mtrix.iter().enumerate() {
        println!("- {:02}: {:?}", i, v);
    }
}
