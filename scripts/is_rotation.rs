fn main() -> () {
    if is_rotation("apple", "pleap") {
        println!("ok");
    }
    if is_rotation("orange", "georan") {
        println!("ok");
    }
    if is_rotation("foo", "bar") {
        println!("ok");
    } else {
        println!("boom");
    }
}

fn is_rotation<'a>(one: &'a str, other: &'a str) -> bool {
    let mut a: String = other.to_owned();
    a.push_str(other);
    return a.contains(one);
}
