// https://github.com/rust-lang-nursery/log
// https://github.com/sebasmagri/env_logger/
// https://docs.rs/env_logger/*/env_logger/
#[macro_use]
extern crate log;
extern crate env_logger;

use std::io::prelude::*;
// use std::fmt;
use std::env;
use std::fs::{self, DirBuilder, File};
use std::io;
// use std::io::Read;
// use std::fs::File;
use std::path::PathBuf;

mod timez;

const BASE_URL: &'static str = "Downloads";

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
    let mut src: PathBuf = [
        home,
        BASE_URL.to_owned(),
        "test".to_owned(),
        "foo.txt".to_owned(),
    ].iter()
        .collect();
    let mut dst = format!("{}/{}/test/{}.foo.txt", home, BASE_URL, timez::datetag());
    // https://doc.rust-lang.org/std/fs/fn.copy.html
    // https://doc.rust-lang.org/std/fs/struct.File.html
    info!("copying {:?} into {}", &src, &dst);
    // https://github.com/rust-lang/rfcs/pull/243
    let r = fs::copy(&src, &dst);
    match &r {
        Ok(n_bytes) => {
            debug!("copied {} bytes from {:?} to {}", n_bytes, &src, &dst);
        }
        Err(e) => {
            error!("cannot copy {:?} into {}: {}", &src, &dst, e);
        }
    };

    debug!("result of the copy: {:?}", r);

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
