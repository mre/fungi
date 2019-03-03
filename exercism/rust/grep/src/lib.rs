// https://rust-lang-nursery.github.io/failure/
// https://docs.rs/failure/0.1.5/failure/
// https://github.com/rust-lang-nursery/failure

#[macro_use]
extern crate failure;

use failure::Error;

// https://docs.rs/bitflags/1.0.4/bitflags/
// https://github.com/bitflags/bitflags
#[macro_use]
extern crate bitflags;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// While using raw slice of str to handle flags is convenient,
/// in the real-world projects it is customary to use a struct,
/// that contains flags-related logic. So in this exercise
/// we ask you to implement a custom struct.
///
/// If you are curious about real-world implementation, refer to the `clap-rs` crate:
/// https://github.com/kbknapp/clap-rs/blob/master/src/args/arg_matches.rs

// Flags:
// - `-n` Print the line numbers of each matching line.
// - `-l` Print only the names of files that contain at least one matching line.
// - `-i` Match line using a case-insensitive comparison.
// - `-v` Invert the program -- collect all lines that fail to match the pattern.
// - `-x` Only match entire lines, instead of lines that contain a match.

bitflags! {
    pub struct Flags: u8 {
        const LINE_NUMBERS = 1 << 0;     // -n // 0b0000_0001
        const LIST_FILE_NAMES = 1 << 1;  // -l // 0b0000_0010
        const CASE_INSENSITIVE = 1 << 2; // -i // 0b0000_0100
        const INVERSE_MATCH = 1 << 3;    // -v // 0b0000_1000
        const EXACT_MATCH = 1 << 4;      // -x // 0b0001_0000
    }
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        let mut f: Flags = Flags::empty();
        for &flag in flags {
            f |= match flag {
                "-n" => Flags::LINE_NUMBERS,
                "-l" => Flags::LIST_FILE_NAMES,
                "-i" => Flags::CASE_INSENSITIVE,
                "-v" => Flags::INVERSE_MATCH,
                "-x" => Flags::EXACT_MATCH,
                _ => continue,
            };
        }
        f
    }
}

#[derive(Debug, Fail)]
enum GrepIOError {
    #[fail(display = "File not found: {}", path)]
    FileNotFoundError { path: String },

    // #[fail(display = "File could not be read: {}", path)]
    // FileReadError { path: String },
}

fn check_file(file: &str) -> Result<(), GrepIOError> {
        let path = Path::new(file);
        if !path.is_file() {
            return Err(GrepIOError::FileNotFoundError {
                path: String::from(file),
             });
        }
    return Ok(());
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let mut result = Vec::new();
    
    if pattern.is_empty() || files.is_empty() {
        return Ok(result);
    }

    let invert: bool = flags.contains(Flags::INVERSE_MATCH);

    for file in files {
        check_file(file)?;
        let fh = File::open(file)?;

        for (n, line) in BufReader::new(fh).lines().enumerate() {
            if let Ok(mut content) = line {
                let display = match (flags.contains(Flags::EXACT_MATCH), flags.contains(Flags::CASE_INSENSITIVE)) {
                    (false, false) => content.contains(pattern),
                    (false, true) => content
                        .to_lowercase()
                        .contains(pattern.to_lowercase().as_str()),
                    (true, false) => content == pattern,
                    (true, true) => content.to_lowercase() == pattern.to_lowercase(),
                };

                if flags.contains(Flags::LINE_NUMBERS) {
                    content = format!("{}:{}", n + 1, content);
                }
                if 1 < files.len() {
                     content = format!("{}:{}", file, content);
                }

                if display && !invert || !display && invert {
                    if flags.contains(Flags::LIST_FILE_NAMES) {
                        result.push(file.to_string());
                        break;
                    } else {
                        result.push(content);
                    }
                }
            }
        }
    }
    return Ok(result);
}
