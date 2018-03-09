// https://doc.rust-lang.org/book/second-edition/ch11-03-test-organization.html

// https://github.com/rust-lang/rfcs/pull/940
// https://github.com/rust-lang/rfcs/blob/master/text/0940-hyphens-considered-harmful.md
// https://doc.rust-lang.org/book/first-edition/macros.html
#[macro_use]
extern crate hello_world_derive as hwd;

trait HelloWorld {
    fn hello_world();
}

trait HelloWorldName {
    fn hello_world_name() -> String;
}

#[derive(HelloWorld, HelloWorldName)]
#[Prefix = "the best"]
struct FrenchToast;

#[derive(HelloWorld, HelloWorldName)]
#[Prefix = "the strange"]
struct Waffles;

#[derive(HelloWorld, HelloWorldName)]
#[Prefix = "the curious"]
struct Pancakes;

#[test]
fn it_works_as_integration_test() {
    assert_eq!(FrenchToast::hello_world_name(), "Hello, World! My name is FrenchToast");
    assert_eq!(Waffles::hello_world_name(), "Hello, World! My name is Waffles");
    assert_eq!(Pancakes::hello_world_name(), "Hello, World! My name is Pancakes");
    
    assert_eq!(true, true);
}
