// https://doc.rust-lang.org/regex/regex/index.html

use regex::Regex;

#[allow(dead_code)]
#[allow(unused_variables)]
fn one() {
    use regex::Regex;
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    assert!(re.is_match("2014-01-01"));
}

// it is an anti-pattern to compile the same regular expression in a loop since
// compilation is typically expensive. (It takes anywhere from a few
// microseconds to a few milliseconds depending on the size of the regex.) Not
// only is compilation itself expensive, but this also prevents optimizations
// that reuse allocations internally to the matching engines.

// In Rust, it can sometimes be a pain to pass regular expressions around if
// they're used from inside a helper function. Instead, we recommend using the
// lazy_static crate to ensure that regular expressions are compiled exactly
// once.

#[allow(dead_code)]
#[allow(unused_variables)]
fn two() {
    // in src/main.rs
    // #[macro_use]
    // extern crate lazy_static;
    // extern crate regex;

    use regex::Regex;

    fn some_helper_function(text: &str) -> bool {
        lazy_static! {
            static ref RE: Regex = Regex::new("...").unwrap();
        }
        RE.is_match(text)
    }

    // Specifically, in this example, the regex will be compiled when it is used for
    // the first time. On subsequent uses, it will reuse the previous compilation.

    let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
    let text = "2012-03-14, 2013-01-01 and 2014-07-05";
    for cap in re.captures_iter(text) {
        println!("Month: {} Day: {} Year: {}", &cap[2], &cap[3], &cap[1]);
    }
    // Output:
    // Month: 03 Day: 14 Year: 2012
    // Month: 01 Day: 01 Year: 2013
    // Month: 07 Day: 05 Year: 2014
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn three() {
    let re = Regex::new(r"(?P<y>\d{4})-(?P<m>\d{2})-(?P<d>\d{2})").unwrap();
    let before = "2012-03-14, 2013-01-01 and 2014-07-05";
    let after = re.replace_all(before, "$m/$d/$y");
    assert_eq!(after, "03/14/2012, 01/01/2013 and 07/05/2014");
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn four() {
    let re = Regex::new(
        r"(?x)
  (?P<y>\d{4}) # the year
  -
  (?P<m>\d{2}) # the month
  -
  (?P<d>\d{2}) # the day
",
    ).unwrap();
    let before = "2012-03-14, 2013-01-01 and 2014-07-05";
    let after = re.replace_all(before, "$m/$d/$y");
    assert_eq!(after, "03/14/2012, 01/01/2013 and 07/05/2014");
}

pub fn sample() {
    one();
    two();
    three();
    four();
}
