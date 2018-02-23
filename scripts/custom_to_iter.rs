// A sample collection, that's just a wrapper over Vec<T>
#[derive(Debug)]
struct MyCollection(Vec<i32>);

// Let's give it some methods so we can create one and add things
// to it.
impl MyCollection {
    fn new() -> MyCollection {
        MyCollection(Vec::new())
    }

    fn add(&mut self, elem: i32) {
        self.0.push(elem);
    }
}

// and we'll implement IntoIterator
impl IntoIterator for MyCollection {
    type Item = i32;
    type IntoIter = ::std::vec::IntoIter<i32>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Debug)]
struct IterStuff {
    len: i32,
}

impl IterStuff {
    fn new(l: i32) -> IterStuff {
        IterStuff { len: l }
    }
}

impl IntoIterator for IterStuff {
    type Item = i32;
    type IntoIter = ::std::vec::IntoIter<i32>;

    fn into_iter(self) -> Self::IntoIter {
        // let rng: std::ops::Range<i32> = (0..self.len);
        // std::vec::Vec
        // https://doc.rust-lang.org/std/vec/struct.Vec.html#method.with_capacity
        let mut vrng: std::vec::Vec<i32> = ::std::vec::Vec::with_capacity((self.len + 1) as usize);
        for i in 0..self.len {
            vrng.push(i);
        }
        vrng.into_iter()
    }
}
// rustc scripts/custom_to_iter.rs --out-dir ./target/ && ./target/custom_to_iter
fn main() {
    println!("Now we can make a new collection...");
    let mut c = MyCollection::new();

    println!("... add some stuff to it ...");
    c.add(0);
    c.add(1);
    c.add(2);

    println!("... and then turn it into an Iterator");
    for (i, n) in c.into_iter().enumerate() {
        assert_eq!(i as i32, n);
    }

    println!("or something silly, from 9 to range");
    let r = IterStuff::new(9);
    // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.enumerate
    for (_, n) in r.into_iter().enumerate() {
        println!("{:?}", n);
    }
}
