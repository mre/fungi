// TODO: group error messages
// TODO: group error types
// TODO: encapsulate worker
// TODO: encapsulate operation
// TODO: check for existing files
// TODO: check for existing directories
// TODO: bubble up errors
// TODO: tar compression
// TODO: symmetric encryption

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
// use std::io::{Error, ErrorKind};

// use std::io::Read;
// use std::fs::File;
use std::path::{Path, PathBuf};

// use std::fmt::Debug;

mod timez;

const BASE_URL: &'static str = "Downloads";

// #[derive(Debug)]
// struct Operation  {
//     name: String,
// }

#[derive(Debug)]
enum Operation {
    CopyFile,
    CopyDir,
}

fn verify_operation<S: AsRef<Path> + std::fmt::Debug, D: AsRef<Path> + std::fmt::Debug>(
    op: Operation,
    src: &S,
    dst: &D,
) -> Result<bool, String> {
    debug!("verifying the result of {:?}", op);
    let mut s_size: u64 = 0;
    let mut d_size: u64 = 0;
    match fs::metadata(src) {
        Ok(md) => {
            // md: fs::Metadata
            s_size = md.len();
            debug!("meta from the source (size: {:?})", s_size);
        }
        Err(e) => {
            error!("cannot get metadata from {:?}: {}", src, e);
        }
    };
    match fs::metadata(dst) {
        Ok(md) => {
            d_size = md.len();
            debug!("meta from the destination (size: {:?})", d_size);
        }
        Err(e) => {
            error!("cannot get metadata from {:?}: {}", dst, e);
        }
    };
    if s_size != d_size {
        error!(
            "something something has been lost performing {:?} on {:?} and {:?}",
            op, src, dst
        );
        return Err(String::from("mismatching_size"));
    }
    info!("operation {:?} on {:?} and {:?} successful", op, src, dst);
    Ok(true)
}

// https://doc.rust-lang.org/std/fs/fn.read_dir.html
// one possible implementation of walking a directory only visiting files
fn visit_dirs<T>(dir: &Path, cb: &T) -> io::Result<()>
where
    T: Fn(&DirEntry) -> Result<bool, io::Error>,
{
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
                        cb(&entry)?;
                    }
                }
            }
            // https://doc.rust-lang.org/std/io/struct.Error.html
            Err(e) => error!("cannot visit dir {:?} {}", &dir, e),
        };
    }
    Ok(())
}

// https://doc.rust-lang.org/std/convert/trait.AsRef.html
// fn is_hello<T: AsRef<str>>(s: T) {
//    assert_eq!("hello", s.as_ref());
// }
//
// let s = "hello";
// is_hello(s);
//
// let s = "hello".to_string();
// is_hello(s);
//

fn tag_name<P>(home: &P, path: &P, name: &P) -> PathBuf
where
    P: AsRef<Path> + std::fmt::Debug + std::fmt::Display,
    // P: AsRef<std::path::Path>,
{
    let b_url: PathBuf = PathBuf::from(BASE_URL);
    [
        home.as_ref(),
        b_url.as_ref(),
        path.as_ref(),
        format!("{}.{}", timez::datetag(), name).as_ref(),
    ].iter()
        .collect()
}

fn create_file_in(dir: &PathBuf) -> Result<PathBuf, io::Error> {
    let mut f_dir: PathBuf = dir.clone();
    f_dir.push("foo.txt");
    let mut file = File::create(&f_dir)?;
    return match file.write_all(b"Hello, world!") {
        Ok(_) => Ok(f_dir),
        Err(e) => Err(e),
    };
}

fn home_name() -> String {
    let home: String = "HOME".to_owned();
    match env::var(home) {
        Ok(h) => h,
        Err(_) => "/".to_owned(),
    }
}

fn copy_file_in<S: AsRef<Path> + std::fmt::Debug, D: AsRef<Path> + std::fmt::Debug>(
    src: S,
    dst: D,
) -> Result<bool, io::Error> {
    // https://doc.rust-lang.org/std/fs/fn.copy.html
    // https://doc.rust-lang.org/std/fs/struct.File.html
    info!("copying {:?} into {:?}", &src, &dst);
    // https://github.com/rust-lang/rfcs/pull/243
    let r = fs::copy(&src, &dst);
    match &r {
        &Ok(n_bytes) => {
            debug!("copied {} bytes from {:?} to {:?}", n_bytes, &src, &dst);
        }
        &Err(ref e) => {
            error!("cannot copy {:?} into {:?}: {:?}", &src, &dst, e);
        }
    };

    match verify_operation(Operation::CopyFile, &src, &dst) {
        Ok(_) => {
            return Ok(true);
        }
        // https://doc.rust-lang.org/std/process/fn.exit.html
        // https://doc.rust-lang.org/std/io/struct.Error.html
        Err(e) => {
            let custom_error = io::Error::new(io::ErrorKind::Other, e);
            return Err(custom_error);
        }
    };
}

fn create_token_file_in(dir: &PathBuf) -> Result<PathBuf, io::Error> {
    create_file_in(&dir)
}

pub fn run() -> Result<bool, io::Error> {
    let home: String = home_name();
    debug!("considering {} as $HOME", home);

    // https://doc.rust-lang.org/std/path/struct.PathBuf.html
    let mut dir: PathBuf = [&home, BASE_URL].iter().collect();
    dir.push("test");

    // https://doc.rust-lang.org/std/fs/struct.DirBuilder.html#method.create
    // Create the specified directory with the options configured in
    // this builder.
    //
    // It is considered an error if the directory already exists unless
    // recursive mode is enabled.
    match DirBuilder::new().recursive(false).create(&dir) {
        // https://doc.rust-lang.org/std/io/type.Result.html
        Ok(_) => {
            info!("directory {:?} created", &dir);
            let f_dir: PathBuf = create_token_file_in(&dir).unwrap();
            debug!("content written in {:?}", f_dir);
        }
        Err(e) => {
            error!(
                "cannot create directory {:?}: {}... destroy (try again)!",
                &dir, e
            );
            // https://doc.rust-lang.org/std/fs/fn.remove_dir.html
            // pub fn remove_dir<P: AsRef<Path>>(path: P) -> Result<()>
            // Removes an existing, empty directory.
            //
            // https://doc.rust-lang.org/std/fs/fn.remove_dir_all.html
            // pub fn remove_dir_all<P: AsRef<Path>>(path: P) -> Result<()>
            // Removes a directory at this path, after removing all its
            // contents. Use carefully!
            fs::remove_dir_all(&dir)?;
            return Err(e);
        }
    };

    debug!("ensuring that {:?} is a directory", &dir);
    assert!(fs::metadata(&dir).unwrap().is_dir());

    // https://doc.rust-lang.org/std/path/struct.Path.html
    // impl AsRef<Path> for String
    //   fn as_ref(&self) -> &Path
    let f_target: &str = "foo.txt";
    let mut src: PathBuf = [&home, BASE_URL, "test", f_target].iter().collect();

    let mut dst: PathBuf = tag_name(&home, &String::from("test"), &String::from(f_target));

    if fs::File::open(&src).is_err() {
        error!("cannot copy {:?} because it's missing", &src);
        let custom_error = io::Error::new(io::ErrorKind::Other, "missing_src");
        return Err(custom_error);
    }
    if fs::File::open(&dst).is_ok() {
        error!("cannot copy into {:?} because it's already there", &dst);
        let custom_error = io::Error::new(io::ErrorKind::Other, "cannot_override");
        return Err(custom_error);
    }

    copy_file_in(&src, &dst)?;

    src = [&home, BASE_URL, "test"].iter().collect();
    dst = [&home, BASE_URL, "tset"].iter().collect();

    if dst.is_dir() {
        error!("directory {:?} is already there, cannot copy!", &dst);
        let custom_error = io::Error::new(io::ErrorKind::Other, "cannot_override");
        return Err(custom_error);
    } else {
        // https://doc.rust-lang.org/std/fs/struct.DirBuilder.html#method.create
        match DirBuilder::new().recursive(true).create(&dst) {
            // https://doc.rust-lang.org/std/io/type.Result.html
            Ok(_) => {
                info!("directory {:?} created", &dst);
                // let mut f_dir: PathBuf = dir.clone();
                // f_dir.push("foo.txt");
                // let mut file = File::create(f_dir)?;
                // file.write_all(b"Hello, world!")?;
            }
            Err(e) => {
                error!("cannot create directory {:?}: {}", &dir, e);
                return Err(e);
            }
        };
    }

    if src.is_dir() {
        visit_dirs(&src, &|f_src| {
            let f_src_path = f_src.path();
            debug!("entering {:?} found {:?}", &src, f_src);
            let f_dst: PathBuf = tag_name(&home, &dst.to_str().unwrap(), &f_src_path);
            debug!("destination filename: {:?}", &f_dst);
            let f_src_s: PathBuf = [src.to_str().unwrap(), &f_src_s].iter().collect();
            debug!("source filename: {:?}", &f_src_s);
            return copy_file_in(f_src_s, f_dst);
        })?;
    }

    let r = fs::copy(&src, &dst);
    match r {
        Ok(n_bytes) => {
            debug!("copied {} bytes from {:?} to {:?}", n_bytes, &src, &dst);
        }
        Err(e) => {
            error!("cannot copy {:?} into {:?}: {:?}", &src, &dst, e);
            let custom_error = io::Error::new(io::ErrorKind::Other, "cannot_copy");
            return Err(custom_error);
        }
    };

    match verify_operation(Operation::CopyDir, &src, &dst) {
        Ok(_) => {}
        // https://doc.rust-lang.org/std/process/fn.exit.html
        // https://doc.rust-lang.org/std/io/struct.Error.html
        Err(e) => {
            let custom_error = io::Error::new(io::ErrorKind::Other, e);
            return Err(custom_error);
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
