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
//!  - http://words.steveklabnik.com/an-overview-of-macros-in-rust

// Macro in DOCTESTS:
// Doc tests automatically wrap the code block in extern crate foo; fn
// main() { … } if they don’t find these elements in the code, but to
// get an exported macro you need the #[macro_use] attribute on the
// extern crate foo;.
//
// Thus, you should write this:
//
// ... /// Usage:
// ... ///
// ... /// ```
// ... /// # #[macro_use] extern crate foo; fn main() {
// ... /// let x = addone!(100);
// ... /// # }
// ... /// ```
//
// #[macro_export]
// macro_rules! addone {
//     ($x:expr) => ($x + 1)
// }
//
// (The lines prefixed with # get hidden in the output, but included,
// sans the marker, in the code that gets compiled for the doc test.)
//
// This is covered in The Rust Programming Language, first edition.
//
// As for std, there is an implied #[macro_use] extern crate std; in all
// crates that lack the #![no_std] crate attribute, so its macros
// immediately work.
//
// - https://doc.rust-lang.org/stable/book/first-edition/documentation.html#documenting-macros
// - https://stackoverflow.com/a/31644342

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
/// trait HelloWorldName {
///     fn hello_world_name() -> String;
/// }
///
/// #[derive(HelloWorld, HelloWorldName)]
/// struct FrenchToast;
///
/// #[derive(HelloWorld, HelloWorldName)]
/// struct Waffles;
///
/// #[derive(HelloWorld, HelloWorldName)]
/// struct Pancakes;
///
/// fn main() {
///     FrenchToast::hello_world();
///     println!("FrenchToast HWN: {}", FrenchToast::hello_world_name());
///
///     Waffles::hello_world();
///     println!("Waffles HWN: {}", Waffles::hello_world_name());
///
///     Pancakes::hello_world();
///     println!("Pancakes HWN: {}", Pancakes::hello_world_name());
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
/// trait HelloWorldName {
///     fn hello_world_name() -> String;
/// }
///
/// #[derive(HelloWorld, HelloWorldName)]
/// #[WorldName = "the best Pancakes"]
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
use std::env;

fn verbose() -> bool {
    let key = "VERBOSE";
    match env::var(key) {
        Ok(_) => true,
        Err(_) => false,
    }
}

/// hello_world
///
/// We are going to take a String of the Rust code for the type we are
/// deriving, parse it using syn, construct the implementation of
/// hello_world (using quote), then pass it back to Rust compiler.
#[proc_macro_derive(HelloWorld, attributes(WorldName))]
pub fn hello_world(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition.
    // `input: TokenSteam` is immediately converted to a String.  This
    //  String is a string representation of the Rust code for which we
    //  are deriving HelloWorld. At the moment, the only thing you can
    //  do with a TokenStream is convert it to a string.
    let s = input.to_string();

    if verbose() {
        println!("[proc_macro] input is {:?}", s);
    }

    // Parse the string representation.
    // syn is a crate for parsing Rust code.
    // quote it's essentially the dual of syn as it will make generating
    // Rust code really easy.
    if verbose() {
        println!("[proc_macro] deriving input...");
    }
    let r_ast = syn::parse_derive_input(&s);
    if r_ast.is_err() {
        let err = r_ast.unwrap_err();
        if verbose() {
            println!("[proc_macro] something went wrong...");
        }
        panic!(err);
    }

    if verbose() {
        println!("[proc_macro] unwrapping the AST");
    }
    let ast = r_ast.unwrap();
    if verbose() {
        println!("[proc_macro] build the impl");
    }
    let gen = impl_hello_world(&ast);
    if verbose() {
        println!("[proc_macro] return the generated impl");
    }
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

#[proc_macro_derive(HelloWorldName, attributes(Prefix))]
pub fn hello_world_name(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let r_ast = syn::parse_derive_input(&s);
    if r_ast.is_err() {
        let err = r_ast.unwrap_err();
        if verbose() {
            println!("[proc_macro] something went wrong...");
        }
        panic!(err);
    }

    let ast = r_ast.unwrap();
    let gen = impl_hello_world_name(&ast);
    gen.parse().unwrap()
}

fn impl_hello_world_name(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    // https://docs.rs/syn/0.11.11/syn/struct.DeriveInput.html
    // let attrs: &Vec<&str> = &ast.attrs.iter().map(|f| f.name()).collect();
    // println!("{:?}", attrs);
    //
    // https://dtolnay.github.io/syn/syn/struct.Attribute.html
    // let p = &ast.attrs.clone().remove(0);
    // println!("{:?}", p);
    if let syn::Body::Struct(_) = ast.body {
        quote! {
            impl HelloWorldName for #name {
                fn hello_world_name() -> String {
                    format!("Hello, World! My name is {}", stringify!(#name))
                }
            }
        }
    } else {
        panic!("#[derive(HelloWorldName)] is only defined for structs, not for enums!");
    }
}

// Quote 0.4.2
// https://docs.rs/quote/0.4.2/quote/

// Syn 0.12.13
// https://dtolnay.github.io/syn/syn/
// https://docs.rs/syn/0.12.13/syn/

// TokenStream
// https://doc.rust-lang.org/proc_macro/struct.TokenStream.html

// ConditionalCompilation
// https://rustbyexample.com/attribute/cfg.html
// https://doc.rust-lang.org/reference/attributes.html#conditional-compilation
// https://rust-lang-nursery.github.io/api-guidelines/macros.html

// https://doc.rust-lang.org/proc_macro/index.html
// extern crate proc_macro;

#[cfg(test)]
mod tests {
    use super::hello_world;
    use super::hello_world_name;

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

    #[test]
    #[ignore]
    fn it_prints_the_expected_string() {
        // https://github.com/rust-lang/rust/blob/e8af0f4c1f121263e55da29854208db0ae1fea54/src/libproc_macro/lib.rs#L865
        // https://github.com/ryoon/rustc-1.19.0/blob/4ca47e69f710b93580101782576ebad2bef64749/src/libproc_macro/lib.rs#L146
        // https://github.com/rust-lang/rust/issues/39870
        hello_world_name(TokenStream::from_str("").unwrap());
    }
}
