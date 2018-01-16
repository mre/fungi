// https://doc.rust-lang.org/stable/book/second-edition/ch05-01-defining-structs.html

#[derive(Debug)]
pub struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

pub fn sample() {
    one();
    two();
}

fn two() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area_naive(width1, height1)
    );

    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_confusing(rect1)
    );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let _sq = Rectangle::square(3);
}

fn one() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    // struct update syntax.
    user1 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    println!("User: {:#?}", user1);

    user1 = build_user(
        String::from("anotheremail@example.com"),
        String::from("anothername"),
    );

    println!("User: {:#?}", user1);

    user1 = build_user_field_init_shorthand(
        String::from("anotheremail@example.com"),
        String::from("anothername"),
    );

    println!("User: {:#?}", user1);

    // tuple structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    struct UnitLikeStruct();
    let _unit_like_struct = UnitLikeStruct();
}

pub fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

pub fn build_user_field_init_shorthand(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn area_naive(width: u32, height: u32) -> u32 {
    width * height
}

fn area_confusing(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

impl Rectangle {
    // Methods can take ownership of self, borrow self immutably as we’ve done
    // here, or borrow self mutably, just like any other parameter.
    // If we wanted to change the instance that we’ve called the method on as
    // part of what the method does, we’d use &mut self as the first parameter.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    // associated function
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// Rust doesn’t have an equivalent to the (C's) -> operator; instead, Rust has a
// feature called automatic referencing and dereferencing.
