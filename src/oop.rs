// https://doc.rust-lang.org/stable/book/second-edition/ch17-01-what-is-oo.html
// https://doc.rust-lang.org/stable/book/second-edition/ch17-02-trait-objects.html

// Is Rust and Object Oriented Programming Language?
// Object-oriented programs are made up of objects. An object packages both data
// and the procedures that operate on that data. The procedures are typically
// called methods or operations.
// Encapsulation: that the implementation details of an object aren't accessible
// to code using that object.
// Inheritance is a mechanism whereby an object can inherit from another
// object's definition, thus gaining the parent object's data and behavior
// without you having to define them again.

pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }

    // We leave the list and average fields private so that there's no way for
    // external code to add or remove items to the list field directly,
    // otherwise the average field might become out of sync when the list
    // changes.
    // If encapsulation is a required aspect for a language to be considered
    // object-oriented, then Rust meets that requirement.
}

// If a language must have inheritance to be an object-oriented language, then
// Rust is not. There is no way to define a struct that inherits the parent
// struct's fields and method implementations.
// To reuse code we can use default implementations from Traits.
//
// Polymorphism
// To many people, polymorphism is synonymous with inheritance. But it's
// actually a more general concept that refers to code that can work with data
// of multiple types. For inheritance, those types are generally subclasses.
// Rust instead uses generics to abstract over different possible types, and
// trait bounds to impose constraints on what those types must provide. This is
// sometimes called bounded parametric polymorphism.

// Defining a trait for common behaviour.
// we can define a vector that takes a trait object. A trait object points to an
// instance of a type that implements the trait we specify. We create a trait
// object by specifying some sort of pointer, such as a & reference or a Box<T>
// smart pointer, and then specifying the relevant trait.
// We can use trait objects in place of a generic or concrete type. Wherever we
// use a trait object, Rust's type system will ensure at compile-time that any
// value used in that context will implement the trait object's trait.

pub trait Draw {
    fn draw(&self);
}

//  a struct named Screen that holds a vector named components. This vector is
//  of type Box<Draw>, which is a trait object: it's a stand-in for any type
//  inside a Box that implements the Draw trait.

pub struct Screen {
    pub components: Vec<Box<Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// This works differently to defining a struct that uses a generic type
// parameter with trait bounds. A generic type parameter can only be substituted
// with one concrete type at a time, while trait objects allow for multiple
// concrete types to fill in for the trait object at runtime. For example, we
// could have defined the Screen struct using a generic type and a trait bound:

pub struct ScreenGenerics<T: Draw> {
    pub components: Vec<T>,
}

impl<T> ScreenGenerics<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// This restricts us to a Screen instance that has a list of components all of
// type Button or all of type TextField. If you'll only ever have homogeneous
// collections, using generics and trait bounds is preferable since the
// definitions will be monomorphized at compile time to use the concrete types.
// With the the method using trait objects, on the other hand, one Screen
// instance can hold a Vec that contains a Box<Button> as well as a
// Box<TextField>.

fn one() {}

pub fn sample() {}
