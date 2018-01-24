// https://doc.rust-lang.org/stable/book/second-edition/ch14-02-publishing-to-crates-io.html

// If the structure of your library isn't convenient for others to use from
// another library, you don't have to rearrange your internal organization: you
// can choose to re-export items to make a public structure that's different to
// your private structure, using pub use. Re-exporting takes a public item in
// one location and makes it public in another location as if it was defined in
// the other location instead.

//! # Art
//!
//! A library for modeling artistic concepts.

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        // ...snip...
        Orange
    }
}

// An example of its usage:
// extern crate art;
//
// use art::kinds::PrimaryColor;
// use art::utils::mix;
//
// fn main() {
//     let red = PrimaryColor::Red;
//     let yellow = PrimaryColor::Yellow;
//     mix(red, yellow);
// }

// Can be convenient for the users of this library to re-export the modules:
//
// //! # Art
// //!
// //! A library for modeling artistic concepts.
// 
// pub use kinds::PrimaryColor;
// pub use kinds::SecondaryColor;
// pub use utils::mix;
// 
// pub mod kinds {
//     // ...snip...
// }
// 
// pub mod utils {
//     // ...snip...
// }

// In this way that's the result from the user prospective.

// extern crate art;
// 
// use art::PrimaryColor;
// use art::mix;
// 
// fn main() {
//     // ...snip...
// }

// In cases where there are many nested modules, re-exporting the types at the
// top level with pub use can make a big difference in the experience of people
// who use the crate.
