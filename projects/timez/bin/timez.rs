#[macro_use]
extern crate structopt;
extern crate timez;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "timez")]
struct Opt {
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: u8,

    // Tag is the option used to request a specific tag; under the hood is
    // using a "char".
    // https://doc.rust-lang.org/std/primitive.char.html
    //
    /// What kind of tag is displayed: [f]ull, [w]eek, [d]ay.
    /// The default value is [f]ull.
    #[structopt(short = "t", long = "tag", default_value = "f")]
    tag: char,
}

fn main() {
    let opt = Opt::from_args();
    if opt.verbose > 0 {
        println!("options given: {:?}", opt);
    }
    match opt.tag {
        'f' => print!("{}", timez::datetag()),
        'w' => print!("{}", timez::week_of_the_year()),
        'd' => print!("{}", timez::day_of_the_year()),
        _ => print!(""),
    }
}
