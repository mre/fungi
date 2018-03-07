// https://doc.rust-lang.org/book/second-edition/ch11-03-test-organization.html

// https://github.com/rust-lang/rfcs/pull/940
// https://github.com/rust-lang/rfcs/blob/master/text/0940-hyphens-considered-harmful.md
#[macro_use]
extern crate hello_world_derive as hwd;

trait HelloWorld {
    fn hello_world();
}

#[derive(HelloWorld)]
#[WorldName = "the best FT"]
struct FrenchToast;

#[derive(HelloWorld)]
#[WorldName = "the best WF"]
struct Waffles;

#[derive(HelloWorld)]
#[WorldName = "the best PC"]
struct Pancakes;

#[test]
fn it_works_as_integration_test() {
    FrenchToast::hello_world();
    Waffles::hello_world();
    Pancakes::hello_world();
    
    assert_eq!(true, true);
}
