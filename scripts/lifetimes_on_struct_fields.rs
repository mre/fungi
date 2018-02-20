struct Person<'a> {
    name: &'a str,
}

impl<'a> Person<'a> {
    fn greet(&self) {
        println!("Hello, my name is {}", self.name);
    }
}

fn main() {
    let person = Person { name: "Adam" };
    person.greet();
}
