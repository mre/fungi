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
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
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
}
