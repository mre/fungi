// https://doc.rust-lang.org/stable/book/second-edition/ch08-01-vectors.html
//
// https://doc.rust-lang.org/stable/std/collections/
// Use a Vec when:
// You want to collect items up to be processed or sent elsewhere later, and don't care about any properties of the actual values being stored.
//     You want a sequence of elements in a particular order, and will only be appending to (or near) the end.
//     You want a stack.
//     You want a resizable array.
//     You want a heap-allocated array.
//
// |     | get(i) | insert(i) | remove(i) | append | split_off(i) |
// |-----+--------+-----------+-----------+--------+--------------|
// | Vec | O(1)   | O(n-i)*   | O(n-i)    | O(m)*  | O(n-i)       |
//
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

    {
        // we can’t have mutable and immutable references in the same scope.
        // let mut v = vec![1, 2, 3, 4, 5];
        // let first = &v[0];
        // v.push(6);
        //
        //     error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
        //         -->
        //       |
        //     4 |     let first = &v[0];
        //                         - immutable borrow occurs here
        //     5 |
        //     6 |     v.push(6);
        //       |     ^ mutable borrow occurs here
        //     7 |
        //     8 | }
        //       | - immutable borrow ends here
    }

    // Note: For more on the implementation details of the Vec<T> type, see “The
    // Nomicon” at https://doc.rust-lang.org/stable/nomicon/vec.html.

    {
        let v = vec![100, 32, 57];
        for i in &v {
            println!("{}", i);
        }
    }

    {
        // To change the value that the mutable reference refers to, we have to
        // use the dereference operator (*) to get to the value in i before we
        // can use the += operator .
        let mut v = vec![100, 32, 57];
        for i in &mut v {
            *i += 50;
        }
    }

    {
        #[derive(Debug)]
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }

        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];

        println!("spreadsheet row is {:?}", row)
    }

    {
        let mut vec1 = vec![1, 2, 3, 4];
        let vec2 = vec![10, 20, 30, 40];
        vec1.extend(vec2);
        println!("{:?}", vec1);
    }

    {
        use std::collections::VecDeque;

        let vec = vec![1, 2, 3, 4];
        let _buf: VecDeque<_> = vec.into_iter().collect();
    }

    // Among the adapters are functional favorites like map, fold, skip and
    // take. Of particular interest to collections is the rev adapter, that
    // reverses any iterator that supports this operation. Most collections
    // provide reversible iterators as the way to iterate over them in reverse
    // order.

    {
        let vec = vec![1, 2, 3, 4];
        for x in vec.iter().rev() {
            println!("vec contained {}", x);
        }
    }
}
