//! # Procedural Macros (and custom Derive)
//!
//! `hello-world-derive` is an exercise to create a simple prodedural macro to
//! provide a custom Derive trait.
//!
//! Rust includes several traits that you can derive, but it also lets
//! you define your own. We can accomplish this task through a feature
//! of Rust called "procedural macros." Eventually, procedural macros
//! will allow for all sorts of advanced metaprogramming in Rust, but
//! today, they're only for custom derive.

// Sources
// Procedural Macros: https://doc.rust-lang.org/book/first-edition/procedural-macros.html
// Commenting: https://doc.rust-lang.org/book/second-edition/ch14-02-publishing-to-crates-io.html

// A Point that have a derived Debug trait.
#[allow(dead_code)]
#[derive(Debug)]
struct DPoint {
    x: i32,
    y: i32,
}

// A Point that has a custom explicitly derived Debug trait.
#[allow(dead_code)]
struct Point {
    x: i32,
    y: i32,
}

use std::fmt;

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point {{ x: {}, y: {} }}", self.x, self.y)
    }
}

/// Examples
///
/// ```
/// #[macro_use]
/// extern crate hello_world_derive;
///
/// trait HelloWorld {
///     fn hello_world();
/// }
///
/// #[derive(HelloWorld)]
/// struct FrenchToast;
///
/// #[derive(HelloWorld)]
/// struct Waffles;
///
/// #[derive(HelloWorld)]
/// struct Pancakes;
///
/// fn main() {
///     FrenchToast::hello_world();
///     Waffles::hello_world();
///     Pancakes::hello_world();
/// }
/// ```
///
/// With Custom attributes:
///
/// ```
/// #[macro_use]
/// extern crate hello_world_derive;
///
/// trait HelloWorld {
///     fn hello_world();
/// }
///
/// #[derive(HelloWorld)]
/// #[HelloWorldName = "the best Pancakes"]
/// struct Pancakes;
///
/// fn main() {
///     Pancakes::hello_world();
/// }
/// ```

extern crate proc_macro;
extern crate syn;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

/// hello_world
///
/// We are going to take a String of the Rust code for the type we are
/// deriving, parse it using syn, construct the implementation of
/// hello_world (using quote), then pass it back to Rust compiler.
#[proc_macro_derive(HelloWorld, attributes(HelloWorldName))]
pub fn hello_world(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition.
    // `input: TokenSteam` is immediately converted to a String.  This
    //  String is a string representation of the Rust code for which we
    //  are deriving HelloWorld. At the moment, the only thing you can
    //  do with a TokenStream is convert it to a string.
    let s = input.to_string();

    // Parse the string representation.
    // syn is a crate for parsing Rust code.
    // quote it's essentially the dual of syn as it will make generating
    // Rust code really easy.
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = impl_hello_world(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}

// The ast argument is a struct that gives us a representation of our
// type (which can be either a struct or an enum).
//   - https://docs.rs/syn/0.11.11/syn/struct.DeriveInput.html
//   - https://docs.rs/quote
// We are able to get the name of the type using ast.ident. The quote!
// macro lets us write up the Rust code that we wish to return and
// convert it into Tokens. quote! lets us use some really cool
// templating mechanics; we simply write #name and quote! will replace
// it with the variable named name.
fn impl_hello_world(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    // Check if derive(HelloWorld) was specified for a struct
    if let syn::Body::Struct(_) = ast.body {
        // Yes, this is a struct
        quote! {
            impl HelloWorld for #name {
                fn hello_world() {
                    println!("Hello, World! My name is {}", stringify!(#name));
                }
            }
        }
    } else {
        //Nope. This is an Enum. We cannot handle these!
        panic!("#[derive(HelloWorld)] is only defined for structs, not for enums!");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works_as_unit_test() {
        assert_eq!(2 + 2, 4);
    }
}
