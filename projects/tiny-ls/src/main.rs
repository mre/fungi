extern crate chrono;
extern crate libc;
#[macro_use]
extern crate structopt;

use std::fs;
use std::path::PathBuf;
use std::error::Error;
use std::process;
use std::os::unix::fs::PermissionsExt;
use libc::{S_IRGRP, S_IROTH, S_IRUSR, S_IWGRP, S_IWOTH, S_IWUSR, S_IXGRP, S_IXOTH, S_IXUSR};

use structopt::StructOpt;
use chrono::{DateTime, Local};

// Crate structopt
// https://docs.rs/structopt/0.2.5/structopt/#how-to-derivestructopt
#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(default_value = ".", parse(from_os_str))]
    path: PathBuf,
}

fn parse_permissions(mode: u16) -> String {
    let user = triplet(mode, S_IRUSR, S_IWUSR, S_IXUSR);
    let group = triplet(mode, S_IRGRP, S_IWGRP, S_IXGRP);
    let other = triplet(mode, S_IROTH, S_IWOTH, S_IXOTH);
    [user, group, other].join("")
}

// For each flag read, write, and execute, it runs a binary & operation
// on mode. The output is matched exhaustively against all possible
// permission patterns.
//
// A bitwise AND takes two equal-length binary representations and
// performs the logical AND operation on each pair of the corresponding
// bits, by multiplying them. Thus, if both bits in the compared
// position are 1, the bit in the resulting binary representation is 1
// (1 × 1 = 1); otherwise, the result is 0 (1 × 0 = 0 and 0 × 0 =
// 0). For example:
//
//     0101 (decimal 5)
// AND 0011 (decimal 3)
//   = 0001 (decimal 1)
//
// The operation may be used to determine whether a particular bit is
// set (1) or clear (0). For example, given a bit pattern 0011 (decimal
// 3), to determine whether the second bit is set we use a bitwise AND
// with a bit pattern containing 1 only in the second bit:
//
//     0011 (decimal 3)
// AND 0010 (decimal 2)
//   = 0010 (decimal 2)
//
// Because the result 0010 is non-zero, we know the second bit in the
// original pattern was set. This is often called bit masking. (By
// analogy, the use of masking tape covers, or masks, portions that
// should not be altered or portions that are not of interest. In this
// case, the 0 values mask the bits that are not of interest.)
//
// The bitwise AND may be used to clear selected bits (or flags) of a
// register in which each bit represents an individual Boolean
// state. This technique is an efficient way to store a number of
// Boolean Values Using As Little Memory As Possible. For example, 0110
// (decimal 6) can be considered a set of four flags, where the first
// and fourth flags are clear (0), and the second and third flags are
// set (1). The second bit may be cleared by using a bitwise AND with
// the pattern that has a zero only in the second bit:
//
//     0110 (decimal 6)
// AND 1101 (decimal 13)
//   = 0100 (decimal 4)
//
// https://en.wikipedia.org/wiki/Bitwise_operation#AND
fn triplet(mode: u16, read: u16, write: u16, execute: u16) -> String {
    match (mode & read, mode & write, mode & execute) {
        (0, 0, 0) => "---",
        (_, 0, 0) => "r--",
        (0, _, 0) => "-w-",
        (0, 0, _) => "--x",
        (_, 0, _) => "r-x",
        (_, _, 0) => "rw-",
        (0, _, _) => "-wx",
        (_, _, _) => "rwx",
    }.to_string()
}

fn main() {
    let opt = Opt::from_args();
    if let Err(ref e) = run(&opt.path) {
        println!("{}", e);
        process::exit(1);
    }
}

fn run(dir: &PathBuf) -> Result<(), Box<Error>> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let metadata = entry.metadata()?;
            let file_name = entry
                .file_name()
                .into_string()
                .or_else(|f| Err(format!("Invalid entry: {:?}", f)))?;

            let size = metadata.len();
            let modified: DateTime<Local> = DateTime::from(metadata.modified()?);
            let mode = metadata.permissions().mode();

            println!(
                "{} {:>5} {} {}",
                parse_permissions(mode as u16),
                size,
                modified.format("%_d %b %H:%M").to_string(),
                file_name
            );
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{parse_permissions};
    
    #[test]
    fn parse_permissions_works_for_ur() {
        let expected: String = "r--------".to_owned();
        let result: String = parse_permissions(0o400);

        assert_eq!(
            result,
            expected,
            "Value expected: `{}` but the result was: `{}`",
            expected,
            result
        );
    }

    #[test]
    fn parse_permissions_works_for_ugor() {
        let expected: String = "r--r--r--".to_owned();
        let result: String = parse_permissions(0o444);

        assert_eq!(
            result,
            expected,
            "Value expected: `{}` but the result was: `{}`",
            expected,
            result
        );
    }

    #[test]
    fn triplet_works() {
        assert_eq!(true, true);
    }
}
