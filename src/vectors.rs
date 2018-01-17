// https://doc.rust-lang.org/stable/book/second-edition/ch08-01-vectors.html
pub fn sample() {
    let _v: Vec<i32> = Vec::new();

    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    {
        // It’s more common to create a Vec<T> that has initial values, and Rust
        // provides the vec! macro for convenience. The macro will create a new
        // vector that holds the values we give it.
        let _v = vec![1, 2, 3, 4];
        // do stuff with v
    } // <- v goes out of scope and is freed here

    {
        let v = vec![1, 2, 3, 4, 5];

        let _third: &i32 = &v[2];
        // returns Option<&T>
        let _third: Option<&i32> = v.get(2);

        let v = vec![1, 2, 3, 4, 5];

        // the first [] method will cause a panic! because it references a
        // nonexistent element.
        // let _does_not_exist = &v[100];
        let _does_not_exist = v.get(100);

        // When the program has a valid reference, the borrow checker enforces
        // the ownership and borrowing rules to ensure this reference and any
        // other references to the contents of the vector remain valid.
    }
}
