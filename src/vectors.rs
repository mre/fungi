// https://doc.rust-lang.org/stable/book/second-edition/ch08-01-vectors.html
pub fn sample() {
    let _v: Vec<i32> = Vec::new();

    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    {
        let _v = vec![1, 2, 3, 4];
        // do stuff with v
    } // <- v goes out of scope and is freed here
}
