// https://doc.rust-lang.org/stable/book/second-edition/ch05-01-defining-structs.html

#[derive(Debug)]
pub struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

pub fn sample() {
    one();

    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area_naive(width1, height1)
    );

    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
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

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
