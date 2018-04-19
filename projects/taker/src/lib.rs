// TODO: group error messages
// TODO: group error types
// TODO: encapsulate worker
// TODO: encapsulate operation
// DONE: check for existing files
// DONE: check for existing directories
// TODO: bubble up errors
// TODO: tar compression
// TODO: symmetric encryption
// TODO: read config from .toml

// https://github.com/rust-lang-nursery/log
// https://github.com/sebasmagri/env_logger/
// https://docs.rs/env_logger/*/env_logger/
#[macro_use]
extern crate log;
extern crate env_logger;
// https://docs.rs/rand/0.5.0-pre.0/rand/
extern crate rand;

#[macro_use]
extern crate serde_derive;
extern crate toml;

use std::error::Error;
use std::io::prelude::*;
use std::iter;
// use std::fmt;
use std::env;
use std::fs::{self, DirBuilder, DirEntry, File, ReadDir};
use std::io;
// use std::io::{Error, ErrorKind};

// use std::io::Read;
// use std::fs::File;
use std::path::{Path, PathBuf, StripPrefixError};

// use std::fmt::Debug;

// https://github.com/rust-lang-nursery/rand/blob/master/src/lib.rs
use rand::distributions::Alphanumeric;
use rand::thread_rng;
use rand::Rng;

// Rust only knows to look in src/lib.rs by default. If we want to add
// more files to our project, we need to tell Rust in src/lib.rs to look
// in other files; this is why mod client needs to be defined in
// src/lib.rs and can't be defined in src/client.rs.
//
// The mod keyword declares a new module. Code within the module appears either
// immediately following this declaration within curly brackets or in another
// file.
//
// The use keyword brings modules, or the definitions inside modules, into
// scope so it's easier to refer to them.

// content of config.rs
mod config;
// content of timez.rs
mod timez;

const BASE_URL: &'static str = "Downloads";

#[derive(Debug)]
enum Operation {
    CopyFile,
    #[allow(dead_code)]
    CopyDir,
}

pub fn config() -> config::Config {
    let cfg_path: String = "TAKER_CFG".to_owned();
    config::parse(match env::var(cfg_path) {
        Ok(h) => h,
        Err(_) => "~/.taker.toml".to_owned(),
    })
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

// Minimal example of an AsRef conversion.
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
    // P: AsRef<Path> + std::fmt::Debug + std::fmt::Display,
    P: AsRef<std::path::Path>,
{
    let b_url: PathBuf = PathBuf::from(BASE_URL);
    let name: &Path = name.as_ref();
    [
        home.as_ref(),
        b_url.as_ref(),
        path.as_ref(),
        format!("{}.{}", timez::datetag(), name.display()).as_ref(),
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
    match env::home_dir() {
        Some(path) => String::from(path.to_str().unwrap()),
        None => "/".to_owned(),
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

// https://github.com/rust-lang-nursery/rand/blob/master/src/lib.rs#L601
// https://github.com/rust-lang-nursery/rand/blob/master/src/lib.rs#L380
// https://github.com/rust-lang-nursery/rand/blob/0.5.0-pre.0/src/distributions/other.rs
fn random_from(seed: &str) -> String {
    let mut rng = thread_rng();
    let distr = &Alphanumeric;
    let chars: String = iter::repeat(())
        .map(|()| rng.sample(distr))
        .take(seed.len())
        .collect();
    return chars;
}

pub fn run(cfg: config::Config) -> Result<bool, io::Error> {
    match cfg.files.len() {
        0 => info!("nothing to do"),
        c => {
            info!("taking {:?} entries", c);

            // check what's the $HOME path here.
            let home: String = home_name();
            debug!("considering {} as $HOME", home);

            // create the target directory where the files will be copied.
            let mut dst: PathBuf = [&home, BASE_URL].iter().collect();
            dst.push("taker_target");
            match DirBuilder::new().recursive(false).create(&dst) {
                Ok(_) => {
                    info!("directory {:?} created", &dst);
                    debug!("ensuring that {:?} is a directory", &dst);
                    assert!(fs::metadata(&dst).unwrap().is_dir());
                }
                Err(e) => {
                    error!(
                        "cannot create directory {:?}: {}... destroy (try again)!",
                        &dst, e
                    );
                    return Err(e);
                }
            };

            // pick up one entry at the time from the given config.
            for f in cfg.files {
                let mut f: &PathBuf = &PathBuf::from(f);
                debug!("considering {:?}", f);

                // let p: std::result::Result<
                //     &std::path::Path,
                //     std::path::StripPrefixError,
                // > = f.strip_prefix("~");

                let stripped = &f.strip_prefix("~");
                match stripped {
                    Ok(p) => {
                        let f = &Path::new(&home_name()).join(p);
                        debug!("expanded path: {:?}", f);
                    }
                    // Err(ref error) if error.kind() == std::path::StripPrefixError => {},
                    Err(e) => error!(
                        "cannot strip (or no need to) $HOME from {:?}: {:?}",
                        &f,
                        e
                    ),
                }

                match fs::metadata(&f) {
                    Ok(meta) => {
                        if meta.is_dir() {
                            debug!("{:?} is a directory", f);

                            info!("copying content of {:?} into {:?}", f, &dst);
                            visit_dirs(&f, &|f_src| {
                                debug!("entering {:?} found {:?}", f, f_src.file_name());

                                let home = PathBuf::from(&home);

                                // TODO: push the original dir name
                                let f_dst: PathBuf =
                                    tag_name(&home, &dst, &PathBuf::from(f_src.file_name()));
                                debug!("destination filename: {:?}", &f_dst);
                                let f_src_s: PathBuf = [&f, &f_src.path()].iter().collect();
                                debug!("source filename: {:?}", &f_src_s);
                                return copy_file_in(f_src_s, f_dst);
                            })?;
                        } else {
                            debug!("{:?} is a file", f);
                            // let f_dst: PathBuf =
                            //     tag_name(&home, &dst, &PathBuf::from(p.file_name()));
                            // debug!("destination filename: {:?}", &f_dst);
                            // let f_src_s: PathBuf = [p, &f_src.path()].iter().collect();
                            // debug!("source filename: {:?}", &f_src_s);
                            // return copy_file_in(f_src_s, f_dst);
                        }
                    }
                    Err(e) => error!("cannot read {:?}: {:?}", &f, e.description()),
                }
            }
        }
    };
    Ok(true)
}

pub fn sample(_: config::Config) -> Result<bool, io::Error> {
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

    // let mut dst: PathBuf = tag_name(&home, &String::from("test"), &String::from(f_target));

    let mut dst_name = random_from("foo");
    dst_name.push_str(".txt");
    let mut dst: PathBuf = [&home, BASE_URL, "test", &dst_name].iter().collect();

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

    // src: mut PathBuf
    if !src.is_dir() || !dst.is_dir() {
        error!(
            "cannot copy {:?} into {:?}: buth must be an existing directory",
            &src, &dst
        );
        let custom_error = io::Error::new(io::ErrorKind::Other, "cannot_copy");
        return Err(custom_error);
    }

    info!("copying content of {:?} into {:?}", src, dst);
    visit_dirs(&src, &|f_src| {
        // let f_src_path = f_src.path();
        debug!("entering {:?} found {:?}", &src, f_src.file_name());

        let home = PathBuf::from(&home);

        let f_dst: PathBuf = tag_name(&home, &dst, &PathBuf::from(f_src.file_name()));
        debug!("destination filename: {:?}", &f_dst);
        let f_src_s: PathBuf = [&src, &f_src.path()].iter().collect();
        debug!("source filename: {:?}", &f_src_s);
        return copy_file_in(f_src_s, f_dst);
    })?;

    // https://doc.rust-lang.org/std/fs/struct.DirEntry.html
    // - if let Ok(ref entries) = &fs::read_dir(&dst) {}
    // - if let Ok(entries) = fs::read_dir(&dst) {}

    if let Ok(entries) = fs::read_dir(&dst) {
        // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.count
        // Consumes the iterator, counting the number of iterations and returning it.
        // let c = entries.count();

        let mut c: usize = 0;

        // https://doc.rust-lang.org/std/fs/struct.ReadDir.html
        // https://doc.rust-lang.org/std/iter/trait.Iterator.html
        // - entries.map(|entry| {})
        // - for entry in &entries {}

        for entry in entries {
            if let Ok(entry) = entry {
                // Here, `entry` is a `DirEntry`.
                if let Ok(metadata) = entry.metadata() {
                    // Now let's show our entry's permissions!
                    debug!(
                        "metadata of {:?}: {:?}",
                        entry.path(),
                        metadata.permissions()
                    );
                } else {
                    error!("couldn't get metadata for {:?}", entry.path());
                }
                c = c + 1;
            }
        }
        info!("inspected {:?} entries in {:?}", c, &dst);
    }

    // https://doc.rust-lang.org/std/fs/struct.DirBuilder.html
    // let path = "/tmp/foo/bar/baz";
    // DirBuilder::new().recursive(true).create(path).unwrap();
    // assert!(fs::metadata(path).unwrap().is_dir());

    Ok(true)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
