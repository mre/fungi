use std::io::{BufRead, BufReader};
use std::fs::File;
use std::env;
use std::path::Path;

fn file_to_lines<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("Couldn't open file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|line| line.expect("Could not parse line"))
        .collect()
}

// Why AsRef<Path>?
// [AsRef...] It looks like it's just sugar for passing an owned value
// to a function that only needs a reference.
// If so, isn't monomorphisation of functions using AsRef generating two
// copies of the function, one taking a reference and the other taking a
// value, when only the first one is actually needed?
// Same goes for Into<String>: is one sacrificing code size for
// convenience when using it?
//
// Yes, this was a known issue with these traits when they were
// proposed. One can eliminate this code-size issue by having the
// function be a thin wrapper around a non-generic one:
//
//   fn foo<T: AsRef<Path>>(x: T) { __foo(x.as_ref()) }
//
// It is possible one day we will accept a sugar for this to avoid
// having to do that (if you even care):
//
//   fn foo(x: ~Path) {  /* use x as if it were an &Path in here */ }
//
// The motivation of these traits is that type mangling is a huge pain
// in the neck, and it's really nice for a function to just magically
// figure it out for you. In particular, if a function expects a
// file-path, we have all of these options:
//
//   impl AsRef<Path> for Path
//   impl AsRef<Path> for OsStr
//   impl AsRef<Path> for OsString
//   impl AsRef<Path> for str
//   impl AsRef<Path> for String
//   impl AsRef<Path> for PathBuf
//
// They all "just work" so you can open("hello.txt"),
// open(&format!("hello-{}.txt", num)), etc. Before deref coercions,
// people were pretty upset about having to pass foo.as_slice()
// everywhere. This is a logical extension of that pattern, but with
// annoying aspect that the implementor needs to do it and now things
// are more generic. I'm a bit ambivalent about that.
//
// The reason it's an ultra-generic trait is so that there isn't an
// explosion of AsPath, AsStr, AsSlice, As... traits. No one needs to
// decide it's a good idea to have a trait to randomly convert to some
// specific type, and no one in turn needs to find out about it. You
// just impl the standard conversions on demand.
//
// source: https://www.reddit.com/r/rust/comments/3ntsbn/whats_the_point_of_asref/cvr5n5f/

// How to access command line parameters?
//
// You can access the command line arguments by using the std::env::args[0]
// or std::env::args_os[1] functions. Both functions return an iterator
// over the arguments. The former iterates over Strings (that are easy
// to work with) but panics if one of the arguments is not valid
// unicode. The latter iterates over OsStrings and never panics.
//
// Note that the first element of the iterator is the name of the
// program itself (this is a convention in all major OSes), so the first
// argument is actually the second iterated element.
//
// An easy way to deal with the result of args is to convert it to a Vec:
//
// ```
// use std::env;
//
// fn main() {
//     let args: Vec<_> = env::args().collect();
//     if args.len() > 1 {
//         println!("The first argument is {}", args[1]);
//     }
// }
// ```
//
// You can use the whole standard iterator toolbox[2] to work with these
// arguments. For example, to retrieve only the first argument:
//
// ```
// use std::env;
//
// fn main() {
//     if let Some(arg1) = env::args().nth(1) {
//         println!("The first argument is {}", arg1);
//     }
// }
// ```
//
// You can find libraries on crates.io[3] for parsing command line arguments:
//
// - docopt[4]: you just write the help message, and the parsing code is
//   generated for you.
// - clap[5]: you describe the options you want to parse using a fluent
//   API. Faster than docopt and gives you more control.
// - getopts[6]: port of the popular C library. Lower-level and even
//   more control.
//
// - [0] https://doc.rust-lang.org/stable/std/env/fn.args.html
// - [1] https://doc.rust-lang.org/stable/std/env/fn.args_os.html
// - [2] https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html
// - [3] https://crates.io/
// - [4] https://crates.io/crates/docopt
// - [5] https://crates.io/crates/clap
// - [6] https://crates.io/crates/getopts
// - [7] https://stackoverflow.com/a/15621897
//

// rustc scripts/read_file_lines.rs --out-dir ./target/ && ./target/read_file_lines
fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        let file_name: &String = &args[1];
        println!("Reading the content by line of {}", file_name);
        let ls: Vec<String> = file_to_lines(file_name);
        println!("The file contains {} lines, just that", ls.len());
    } else {
        println!("USAGE: {} [filename]", args[0]);
    }
}
