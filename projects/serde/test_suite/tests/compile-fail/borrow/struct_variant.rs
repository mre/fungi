// Copyright 2017 Serde Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate serde_derive;

#[derive(Deserialize)]
struct Str<'a>(&'a str);

#[derive(Deserialize)] //~ ERROR: proc-macro derive panicked
enum Test<'a> {
    #[serde(borrow)] //~^^ HELP: #[serde(borrow)] may only be used on newtype variants
    S { s: Str<'a> }
}

fn main() {}
