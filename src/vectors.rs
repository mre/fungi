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
// https://doc.rust-lang.org/stable/std/vec/struct.Vec.html

fn one() {
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

fn two() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);

    assert_eq!(vec.len(), 2);
    assert_eq!(vec[0], 1);

    assert_eq!(vec.pop(), Some(2));
    assert_eq!(vec.len(), 1);

    vec[0] = 7;
    assert_eq!(vec[0], 7);

    vec.extend([1, 2, 3].iter().cloned());

    for x in &vec {
        println!("{}", x);
    }
    assert_eq!(vec, [7, 1, 2, 3]);

    let mut vec = vec![1, 2, 3];
    vec.push(4);
    assert_eq!(vec, [1, 2, 3, 4]);

    // It can also initialize each element of a Vec<T> with a given value:
    let vec = vec![0; 5];
    assert_eq!(vec, [0, 0, 0, 0, 0]);

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        // Prints 3, 2, 1
        println!("{}", top);
    }
}

fn three() {
    // A Vec can be mutable. Slices, on the other hand, are read-only objects.
    // To get a slice, use &. Example:

    fn read_slice(_slice: &[usize]) {
        // ...
    }

    let v = vec![0, 1];
    read_slice(&v);

    // ... and that's all!
    // you can also do it like this:
    let _x: &[usize] = &v;
    // In Rust, it's more common to pass slices as arguments rather than vectors
    // when you just want to provide a read access.
}

fn four() {
    // Vec is and always will be a (pointer, capacity, length) triplet. No more,
    // no less.

    // However, the pointer may not actually point to allocated memory. In
    // particular, if you construct a Vec with capacity 0 via Vec::new, vec![],
    // Vec::with_capacity(0), or by calling shrink_to_fit on an empty Vec, it
    // will not allocate memory. Similarly, if you store zero-sized types inside
    // a Vec, it will not allocate space for them. Note that in this case the
    // Vec may not report a capacity of 0. Vec will allocate if and only if
    // mem::size_of::<T>() * capacity() > 0.

    let mut vec = Vec::with_capacity(10);

    // The vector contains no items, even though it has capacity for more
    assert_eq!(vec.len(), 0);

    // These are all done without reallocating...
    for i in 0..10 {
        vec.push(i);
    }

    // ...but this may make the vector reallocate
    vec.push(11);
    println!("{:?}", vec);

    let mut vec = vec![1, 2, 3];
    vec.insert(1, 4);
    assert_eq!(vec, [1, 4, 2, 3]);

    vec.insert(4, 5);
    assert_eq!(vec, [1, 4, 2, 3, 5]);

    let mut v = vec![1, 2, 3];
    assert_eq!(v.remove(1), 2);
    assert_eq!(v, [1, 3]);

    // fn retain<F>(&mut self, f: F)
    // where F: FnMut(&T) -> bool,
    //     Retains only the elements specified by the predicate.
    //     In other words, remove all elements e such that f(&e) returns false.
    //     This method operates in place and preserves the order of the retained
    //     elements.

    let mut vec = vec![1, 2, 3, 4];
    vec.retain(|&x| x % 2 == 0);
    assert_eq!(vec, [2, 4]);

    // fn dedup_by_key<F, K>(&mut self, key: F)
    // where F: FnMut(&mut T) -> K, K: PartialEq<K>,
    // Removes all but the first of consecutive elements in the vector that resolve to the same key.
    // If the vector is sorted, this removes all duplicates.

    let mut vec = vec![10, 20, 21, 30, 20];
    vec.dedup_by_key(|i| *i / 10);
    assert_eq!(vec, [10, 20, 30, 20]);

    // fn dedup_by<F>(&mut self, same_bucket: F)
    // where F: FnMut(&mut T, &mut T) -> bool,

    // Removes all but the first of consecutive elements in the vector satisfying a given equality relation.
    // The same_bucket function is passed references to two elements from the vector, and returns true if the elements compare equal, or false if they do not. The elements are passed in opposite order from their order in the vector, so if same_bucket(a, b) returns true, a is removed.
    // If the vector is sorted, this removes all duplicates.

    let mut vec = vec!["foo", "bar", "Bar", "baz", "bar"];
    vec.dedup_by(|a, b| a.eq_ignore_ascii_case(b));
    assert_eq!(vec, ["foo", "bar", "baz", "bar"]);

    // fn pop(&mut self) -> Option<T>
    //     Removes the last element from a vector and returns it, or None if it is empty.

    let mut vec = vec![1, 2, 3];
    assert_eq!(vec.pop(), Some(3));
    assert_eq!(vec, [1, 2]);

    // fn append(&mut self, other: &mut Vec<T>)
    //     Moves all the elements of other into Self, leaving other empty.
    //     Panics if the number of elements in the vector overflows a usize.

    let mut vec = vec![1, 2, 3];
    let mut vec2 = vec![4, 5, 6];
    vec.append(&mut vec2);
    assert_eq!(vec, [1, 2, 3, 4, 5, 6]);
    assert_eq!(vec2, []);

    // fn drain<R>(&mut self, range: R) -> Drain<T>
    // where R: RangeArgument<usize>,
    // Creates a draining iterator that removes the specified range in the vector and yields the removed items.
    // Note 1: The element range is removed even if the iterator is only partially consumed or not consumed at all.
    // Note 2: It is unspecified how many elements are removed from the vector if the Drain value is leaked.
    // Panics if the starting point is greater than the end point or if the end point is greater than the length of the vector.

    let mut v = vec![1, 2, 3];
    let u: Vec<_> = v.drain(1..).collect();
    assert_eq!(v, &[1]);
    assert_eq!(u, &[2, 3]);

    // A full range clears the vector
    v.drain(..);
    assert_eq!(v, &[]);

    // fn clear(&mut self)
    // Clears the vector, removing all values.
    // Note that this method has no effect on the allocated capacity of the vector.

    let mut v = vec![1, 2, 3];
    v.clear();
    assert!(v.is_empty());

    // fn len(&self) -> usize[src][−]
    // Returns the number of elements in the vector, also referred to as its 'length'.

    let a = vec![1, 2, 3];
    assert_eq!(a.len(), 3);

    // fn is_empty(&self) -> bool[src][−]
    // Returns true if the vector contains no elements.

    let mut v = Vec::new();
    assert!(v.is_empty());

    v.push(1);
    assert!(!v.is_empty());

    // fn split_off(&mut self, at: usize) -> Vec<T>
    // Splits the collection into two at the given index.
    // Returns a newly allocated Self. self contains elements [0, at), and the returned Self contains elements [at, len).
    // Note that the capacity of self does not change.
    // Panics if at > len.

    let mut vec = vec![1, 2, 3];
    let vec2 = vec.split_off(1);
    assert_eq!(vec, [1]);
    assert_eq!(vec2, [2, 3]);

    // fn dedup(&mut self)
    //     Removes consecutive repeated elements in the vector.
    //     If the vector is sorted, this removes all duplicates.

    let mut vec = vec![1, 2, 2, 3, 2];
    vec.dedup();
    assert_eq!(vec, [1, 2, 3, 2]);
}

fn sum(arr: &[f64]) -> f64 {
    arr.iter().fold(0.0, |p, &q| p + q)
}

fn mean(arr: &[f64]) -> f64 {
    sum(arr) / arr.len() as f64
}

pub fn sample() {
    one();
    two();
    three();
    four();

    let v = &[1.0, 2.0, 3.0, 4.0, 5.0, 10.0, 20.0, 40.0];
    println!("mean of {:?}: {:?}", v, mean(v));

    let w = &[];
    println!("mean of {:?}: {:?}", w, mean(w));
}
