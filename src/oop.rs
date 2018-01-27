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

// Implementing the Trait
// Add some types that implement the Draw trait.
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // Code to actually draw a button
    }
}

// extern crate rust_gui;
// use rust_gui::Draw;

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // Code to actually draw a select box
    }
}

// The user of our library can now write their functions to create a Screen
// instance. To this they can add a SelectBox and a Button by putting each in a
// Box<T> to become a trait object. They can then call the run method on the
// Screen instance, which will call draw on each of the components.

// use rust_gui::{Screen, Button};

fn one() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
        // Attempting to use a type that doesn't implement the trait object's
        // trait:
        // Box::new(String::from("Hi")),
        // error[E0277]: the trait bound `std::string::String: Draw` is not
        // satisfied.
    };

    screen.run();
}

// This concept---of being concerned only with the messages a value responds to,
// rather than the value's concrete type---is similar to a concept in
// dynamically typed languages called duck typing:

// By specifying Box<Draw> as the type of the values in the components vector,
// we've defined Screen to need values that we can call the draw method on.

// monomorphization process performed by the compiler when we use trait bounds
// on generics: the compiler generates non-generic implementations of functions
// and methods for each concrete type that we use in place of a generic type
// parameter.
// The code that results from monomorphization is doing static dispatch. Static
// dispatch is when the compiler knows what method you're calling at compile
// time.
// This is opposed to dynamic dispatch, when the compiler can't tell at compile
// time which method you're calling. In these cases, the compiler emits code
// that will figure out at runtime which method to call.
// When we use trait objects, Rust has to use dynamic dispatch. ... it doesn't
// know which method implemented on which type to call. Instead, Rust uses the
// pointers inside of the trait object at runtime to know which specific method
// to call. There's a runtime cost when this lookup happens, compared to static
// dispatch.
// Dynamic dispatch also prevents the compiler from choosing to inline a
// method's code which in turn prevents some optimizations.

// Object Safety is Required for Trait Objects
//
// Only object safe traits can be made into trait objects. There are some
// complex rules around all the properties that make a trait object safe, but in
// practice, there are only two rules that are relevant. A trait is object safe
// if all of the methods defined in the trait have the following properties:
//
// - The return type isn't Self
// - There aren't any generic type parameters
// - The Self keyword is an alias for the type we're implementing traits or
//   methods on.
//
// Object safety is required for trait objects because once you have a trait
// object, you no longer know what the concrete type implementing that trait is.
// If a trait method returns the concrete Self type, but a trait object forgets
// the exact type that it is, there's no way that the method can use the
// original concrete type that it's forgotten.
// An example of a trait whose methods are not object safe is the standard
// library's Clone trait. The signature for the clone method in the Clone trait
// looks like this:
//
// pub trait Clone {
//     fn clone(&self) -> Self;
// }
// if we had tried to implement the Screen struct to hold types that implement
// the Clone trait instead of the Draw trait, like this:
//
// pub struct Screen {
//     pub components: Vec<Box<Clone>>,
// }
// We'll get this error:
//
// error[E0038]: the trait `std::clone::Clone` cannot be made into an object

pub fn sample() {}
