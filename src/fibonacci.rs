use std::mem;

fn fibonacci_iterative() {
    let mut prev = 0;
    let mut curr = 1usize;

    while let Some(n) = curr.checked_add(prev) {
        prev = curr;
        curr = n;
        println!("{}", n);
    }
}

fn fibonacci_recursive(mut prev: usize, mut curr: usize) {
    mem::swap(&mut prev, &mut curr);
    if let Some(n) = curr.checked_add(prev) {
        println!("{}", n);
        fibonacci(prev, n);
    }
}

fn main() {
    fibonacci_recursive(0, 1);
}
