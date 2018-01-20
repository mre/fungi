// https://github.com/rust-lang/rust-by-example/issues/390
pub fn sample() {
    let y = 'y';

    // `ref` on the left side of an assignment is like adding `&` on the right side
    let ref x1 = y;
    let x2 = &y;

    println!("{}", x1 == x2);
}
