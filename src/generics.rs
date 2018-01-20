// https://doc.rust-lang.org/stable/book/second-edition/ch10-00-generics.html

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// We would read this as: the function largest is generic over some type T. It
// has one parameter named list, and the type of list is a slice of values of
// type T. The largest function will return a value of the same type T.
//
// The standard library has defined the trait std::cmp::PartialOrd that types
// can implement to enable comparisons.
// https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html
//
// fn largest<T: PartialOrd>(list: &[T]) -> T {
// But more is needed:
//  |     let mut largest = list[0];
//  |         -----------   ^^^^^^^ cannot move out of here
// The key to this error is cannot move out of type [T], a non-copy array. With
// our non-generic versions of the largest function, we were only trying to find
// the largest i32 or char. As we discussed in Chapter 4, types like i32 and
// char that have a known size can be stored on the stack, so they implement the
// Copy trait. When we changed the largest function to be generic, it’s now
// possible that the list parameter could have types in it that don’t implement
// the Copy trait, which means we wouldn’t be able to move the value out of
// list[0] and into the largest variable.
//
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// If we don’t want to restrict our largest function to only types that
// implement the Copy trait, we could specify that T has the trait bound Clone
// instead of Copy and clone each value in the slice when we want the largest
// function to have ownership. Using the clone function means we’re potentially
// making more heap allocations, though, and heap allocations can be slow if
// we’re working with large amounts of data.
//
// https://doc.rust-lang.org/std/clone/index.html
fn largest_with_clone<T: PartialOrd + Clone>(list: &[T]) -> T {
    // here `list` is a ref to [T]
    // but `T` implements the `Clone` trait, so we can have our own copy of each
    // item (not only references).
    let mut largst: T = list[0].clone();

    for item in list.iter() {
        // iterating `list` gives us references to each item.
        let itm: &T = item;
        // we can compare the reference to `itm` to a reference of largst or
        // clone each item for the comparison.
        if itm > &largst {
            largst = item.clone();
        }
    }

    largst
}

// Another way we could implement largest is for the function to return a
// reference to a T value in the slice. If we change the return type to be &T
// instead of T and change the body of the function to return a reference, we
// wouldn’t need either the Clone or Copy trait bounds and we wouldn’t be doing
// any heap allocations.
fn largest_with_ref<T: PartialOrd>(list: &[T]) -> &T {
    // let mut x = &y creates a pointer to y which you can change to point to
    //                something else. That is, the pointer itself is mutable.
    //
    // let ref mut x = y creates a pointer to a mutable y. You can use it to
    //                   modify y, but it cannot be changed to point to a
    //                   different value. It's identical to let x = &mut y.
    //
    // let _: () = largst; => largst :&T
    let mut largst = &list[0];

    for item in list.iter() {
        if item > largst {
            largst = item;
        }
    }

    largst
}

#[allow(dead_code)]
struct Point<T> {
    x: T,
    y: T,
}

// Note that we have to declare T just after impl in order to use T in the type
// Point<T>. Declaring T as a generic type after the impl is how Rust knows the
// type in the angle brackets in Point is a generic type rather than a concrete
// type.
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Building an impl block which only applies to a struct with a specific type is
// used for the generic type parameter T.
// This code means the type Point<f32> will have a method named
// distance_from_origin, and other instances of Point<T> where T is not of type
// f32 will not have this method defined. This method measures how far our point
// is from the point of coordinates (0.0, 0.0) and uses mathematical operations
// which are only available for floating-point types.
impl Point<f32> {
    fn _distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// enum Option<T> {
//     Some(T),
//     None,
// }
//
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

struct PointM<T, U> {
    x: T,
    y: U,
}

impl<T, U> PointM<T, U> {
    fn mixup<V, W>(self, other: PointM<V, W>) -> PointM<T, W> {
        PointM {
            x: self.x,
            y: other.y,
        }
    }
}

pub fn sample() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    let result = largest(&number_list);
    println!("The largest number is {}", result);
    let result = largest(&char_list);
    println!("The largest char is {}", result);
    let result = largest_with_clone(&number_list);
    println!("The largest number is {}", result);
    let result = largest_with_ref(&number_list);
    println!("The largest number is {}", result);

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    println!("p.x = {}", integer.x());
    println!("p.x = {}", float.x());

    let p1 = PointM { x: 5, y: 10.4 };
    let p2 = PointM { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

// the way that Rust has implemented generics means that your code will not run
// any slower than if you had specified concrete types instead of generic type
// parameters!
//
// Rust accomplishes this by performing monomorphization of code using generics
// at compile time. Monomorphization is the process of turning generic code into
// specific code with the concrete types that are actually used filled in.
//
// What the compiler does is the opposite of the steps that we performed to
// create the generic function. The compiler looks at all the places that
// generic code is called and generates code for the concrete types that the
// generic code is called with.
