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
//!
//! Sources:
//!  - Procedural Macros: https://doc.rust-lang.org/book/first-edition/procedural-macros.html
//!  - Commenting: https://doc.rust-lang.org/book/second-edition/ch14-02-publishing-to-crates-io.html
//!  - https://doc.rust-lang.org/nightly/unstable-book/language-features/proc-macro.html
//!  - https://doc.rust-lang.org/proc_macro/index.html

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

/// Examples:
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

    println!("[proc_macro] input is {:?}", s);

    // Parse the string representation.
    // syn is a crate for parsing Rust code.
    // quote it's essentially the dual of syn as it will make generating
    // Rust code really easy.
    println!("[proc_macro] deriving input...");
    let r_ast = syn::parse_derive_input(&s);
    if r_ast.is_err() {
        let err = r_ast.unwrap_err();
        println!("[proc_macro] something went wrong...");
        panic!(err);
    }
    
    println!("[proc_macro] unwrapping the AST");
    let ast = r_ast.unwrap();
    println!("[proc_macro] build the impl");
    let gen = impl_hello_world(&ast);

    println!("[proc_macro] return the generated impl");
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
        quote! {
            impl HelloWorld for #name {
                fn hello_world() {
                    println!("Hello, World! My name is {}", stringify!(#name));
                }
            }
        }
    } else {
        panic!("#[derive(HelloWorld)] is only defined for structs, not for enums!");
    }
}

// Quote 0.4.2
// https://docs.rs/quote/0.4.2/quote/

// Syn 0.12.13
// https://dtolnay.github.io/syn/syn/
// https://docs.rs/syn/0.12.13/syn/

// TokenStream
// https://doc.rust-lang.org/proc_macro/struct.TokenStream.html

#[cfg(test)]
mod tests {
    #[macro_use]
    use super::{hello_world};
    // extern crate hello_world_derive;

    // https://doc.rust-lang.org/proc_macro/index.html
    extern crate proc_macro;

    use std::str::FromStr;
    use quote::Tokens;
    use proc_macro::TokenStream;

    #[test]
    fn it_works_as_unit_test() {
        assert_eq!(true, true);
    }

    #[test]
    fn it_creates_strings() {
        let s = "5";
        let x = i32::from_str(s).unwrap();

        assert_eq!(5, x);
    }

    // ignoring because:
    // RUST_TEST_THREADS=1 cargo test -- --nocapture
    // thread 'tests::it_uses_token_streams_correctly' panicked at
    // 'proc_macro::__internal::with_sess() called before
    // set_parse_sess()!', libproc_macro/lib.rs:864:9
    #[test]
    #[ignore]
    fn it_uses_token_streams_correctly() {
        // LexError
        // https://doc.rust-lang.org/proc_macro/struct.LexError.html
        // let r: Result<TokenStream, LexError> = TokenStream::from_str("");
        // let e: LexError = r.unwrap_err();
        // println!("err is {:?}", e);

        // ProcMacro - TokenStream
        // https://doc.rust-lang.org/proc_macro/index.html
        // https://doc.rust-lang.org/proc_macro/struct.TokenStream.html

        // #[derive(HelloWorld)]
        // struct Foos;

        let mut tokens = Tokens::new();
        tokens.append("#[HelloWorldName = \"the Foos\"]\nstruct Foos;".to_string());
        println!("TSs are {:?}", tokens);

        // Enum std::result::Result - MapErr
        // pub fn map_err<F, O>(self, op: O) -> Result<T, F>
        // where O: FnOnce(E) -> F,
        // Maps a Result<T, E> to Result<T, F> by applying a function to
        // a contained Err value, leaving an Ok value untouched.
        // This function can be used to pass through a successful result
        // while handling an error.
        //   -  https://doc.rust-lang.org/std/result/enum.Result.html#method.map_err

        let r = TokenStream::from_str(&tokens.to_string())
            .map_err(|e| format!("Et tu, Brute? {:?}", e));

        println!("TSr is {:?}", r);

        assert_eq!(true, true);
        assert_eq!(r.is_ok(), true);
    }

    #[test]
    #[ignore]
    fn it_prints_the_expected_greeting() {
        let mut tokens = Tokens::new();
        tokens.append("Point { x: 2, y: 3 }".to_string());
        let r = TokenStream::from_str(&tokens.to_string())
            .map_err(|e| format!("Et tu, Brute? {:?}", e));
        assert_eq!(r.is_ok(), true);
        hello_world(r.unwrap());
    }
}
