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
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

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
    fn distance_from_origin(&self) -> f32 {
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
