// https://github.com/rust-lang-nursery/log
// https://github.com/sebasmagri/env_logger/
// https://docs.rs/env_logger/*/env_logger/
#[macro_use]
extern crate log;
extern crate env_logger;

use std::io::prelude::*;
// use std::fmt;
use std::env;
use std::fs::{self, DirBuilder, DirEntry, File, ReadDir};
use std::io;
// use std::io::Read;
// use std::fs::File;
use std::path::{Path, PathBuf};

mod timez;

const BASE_URL: &'static str = "Downloads";

// https://doc.rust-lang.org/std/fs/fn.read_dir.html
// one possible implementation of walking a directory only visiting files
fn visit_dirs(dir: &Path, cb: &Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        // https://doc.rust-lang.org/std/fs/fn.read_dir.html
        // pub fn read_dir<P: AsRef<Path>>(path: P) -> Result<ReadDir>
        let r_rds: io::Result<ReadDir> = fs::read_dir(dir);
        match r_rds {
            Ok(entries) => {
                for entry in entries {
                  let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    visit_dirs(&path, cb)?;
                } else {
                    cb(&entry);
                }
                }
            },
            // https://doc.rust-lang.org/std/io/struct.Error.html
            Err(e) => { error!("cannot visit dir {:?} {}", &dir, e) },
        };
    }
    Ok(())
}

pub fn run() -> Result<bool, io::Error> {
    let mut home = "HOME".to_owned();
    home = match env::var(home) {
        Ok(h) => h,
        Err(_) => "/".to_owned(),
    };
    info!("considering {} as $HOME", home);
    // https://doc.rust-lang.org/std/path/struct.PathBuf.html
    let mut dir: PathBuf = [&home, BASE_URL].iter().collect();
    dir.push("test");

    // https://doc.rust-lang.org/std/fs/struct.DirBuilder.html#method.create
    match DirBuilder::new().recursive(true).create(&dir) {
        // https://doc.rust-lang.org/std/io/type.Result.html
        Ok(_) => {
            info!("directory {:?} created", &dir);
            let mut f_dir: PathBuf = dir.clone();
            f_dir.push("foo.txt");
            let mut file = File::create(f_dir)?;
            file.write_all(b"Hello, world!")?;
        }
        Err(e) => error!("cannot create directory {:?}: {}", &dir, e),
    };

    assert!(fs::metadata(&dir).unwrap().is_dir());

    // https://doc.rust-lang.org/std/path/struct.Path.html
    // impl AsRef<Path> for String
    //   fn as_ref(&self) -> &Path
    let mut src: PathBuf = [&home, BASE_URL, "test", "foo.txt"].iter().collect();

    let mut dst: PathBuf = [
        &home,
        BASE_URL,
        "test",
        format!("{}.foo.txt", timez::datetag()).as_ref(),
    ].iter()
        .collect();
    // https://doc.rust-lang.org/std/fs/fn.copy.html
    // https://doc.rust-lang.org/std/fs/struct.File.html
    info!("copying {:?} into {:?}", &src, &dst);
    // https://github.com/rust-lang/rfcs/pull/243
    let mut r = fs::copy(&src, &dst);
    match &r {
        &Ok(n_bytes) => {
            debug!("copied {} bytes from {:?} to {:?}", n_bytes, &src, &dst);
        }
        &Err(ref e) => {
            error!("cannot copy {:?} into {:?}: {:?}", &src, &dst, e);
        }
    };

    debug!("result of the copy: {:?}", r);

    src = [&home, BASE_URL, "test"].iter().collect();
    dst = [&home, BASE_URL, "tset"].iter().collect();

    r = fs::copy(&src, &dst);
    match r {
        Ok(n_bytes) => {
            debug!("copied {} bytes from {:?} to {:?}", n_bytes, &src, &dst);
        }
        Err(e) => {
            error!("cannot copy {:?} into {:?}: {:?}", &src, &dst, e);
        }
    };

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
