fn main() -> () {
    let mut v: Vec<u32> = vec![9, 3, 4, 5, 1, 5, 7, 2, 0];
    println!("v: {:?} ({:?})", v, sorted(&v));
    for _ in 0..v.len() {
        for j in 0..v.len()-1 {
            if v[j] > v[j + 1] {
                v.swap(j, j + 1);
            }
        }
    }
    println!("v: {:?} ({:?})", v, sorted(&v));
}

use std::cmp::Ordering;
use std::mem;

// https://stackoverflow.com/a/28294764
// https://doc.rust-lang.org/std/primitive.slice.html#method.split_at_mut
// https://stackoverflow.com/a/30075629
// let mut v = vec![1, 2, 3];
// let (a, b) = v.split_at_mut(1);   // Returns (&mut [1], &mut [2, 3])
// change(&mut a[0], &mut b[0]);
#[allow(dead_code)]
fn swap_safe<T>(x: &mut [T], i: usize, j: usize) {
    let (lo, hi) = match i.cmp(&j) {
        // no swapping necessary
        Ordering::Equal => return,

        // get the smallest and largest of the two indices
        Ordering::Less => (i, j),
        Ordering::Greater => (j, i),
    };

    let (init, tail) = x.split_at_mut(hi);
    mem::swap(&mut init[lo], &mut tail[0]);
}

fn sorted(a: &Vec<u32>) -> bool {
    for i in 0..a.len()-1 {
        if a[i] > a[i + 1] {
            return false;
        }
    }
    return true;
}
