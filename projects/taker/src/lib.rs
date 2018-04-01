// https://github.com/rust-lang-nursery/log
// https://github.com/sebasmagri/env_logger/
// https://docs.rs/env_logger/*/env_logger/
#[macro_use]
extern crate log;
extern crate env_logger;

// use std::fmt;
use std::fs::{self, DirBuilder};
use std::io;
use std::env;
// use std::io::Read;
// use std::fs::File;

mod timez;

const BASE_URL: &'static str = "Downloads";

pub fn run() -> Result<bool, io::Error> {
    let mut home = "HOME".to_owned();
    home = match env::var(home) {
        Ok(h) => h,
        Err(_) => "/".to_owned(),
    };
    info!("considering {} as $HOME", home);
    let mut src = format!("{}/{}/test/foo.txt", home, BASE_URL);
    let mut dst = format!("{}/{}/test/{}.foo.txt", home, BASE_URL, timez::datetag());    
    // https://doc.rust-lang.org/std/fs/fn.copy.html
    // https://doc.rust-lang.org/std/fs/struct.File.html
    info!("copying {} into {}", src, dst);
    fs::copy(src, dst)?;

    src = format!("{}/{}/test/", home, BASE_URL);
    dst = format!("{}/{}/tset/", home, BASE_URL);
    fs::copy(src, dst)?;
    // https://doc.rust-lang.org/std/fs/struct.DirEntry.html
    if let Ok(entries) = fs::read_dir(".") {
        for entry in entries {
            if let Ok(entry) = entry {
                // Here, `entry` is a `DirEntry`.
                if let Ok(metadata) = entry.metadata() {
                    // Now let's show our entry's permissions!
                    println!("{:?}: {:?}", entry.path(), metadata.permissions());
                } else {
                    println!("Couldn't get metadata for {:?}", entry.path());
                }
            }
        }
    }

    // https://doc.rust-lang.org/std/fs/struct.DirBuilder.html
    let path = "/tmp/foo/bar/baz";
    DirBuilder::new().recursive(true).create(path).unwrap();

    assert!(fs::metadata(path).unwrap().is_dir());
    Ok(true)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
