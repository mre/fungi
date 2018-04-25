// TODO: group error messages
// TODO: group error types
// TODO: encapsulate worker
// DONE: encapsulate operation
// DONE: check for existing files
// DONE: check for existing directories
// TODO: bubble up errors
// DONE: tar compression
// TODO: symmetric encryption
// DONE: read config from .toml
// TODO: remove "home" parameter from create_dir

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
// use std::io::prelude::*;
use std::iter;
// use std::fmt;
use std::env;
use std::fs::{self, DirBuilder};
use std::io;
// use std::io::{Error, ErrorKind};

// use std::io::Read;
// use std::fs::File;
// use std::path::{StripPrefixError};
use std::path::{Path, PathBuf};

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

mod compress;
mod config;
mod encrypter;
mod timez;
mod walkers;

const BASE_URL: &'static str = "Downloads";
const ARCHIVE_NAME: &'static str = "takenfiles";
const COPY_DEST: &'static str = "takentarget";

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

// https://github.com/rust-lang-nursery/rand/blob/master/src/lib.rs#L601
// https://github.com/rust-lang-nursery/rand/blob/master/src/lib.rs#L380
// https://github.com/rust-lang-nursery/rand/blob/0.5.0-pre.0/src/distributions/other.rs
#[allow(dead_code)]
fn random_from(seed: &str) -> String {
    let mut rng = thread_rng();
    let distr = &Alphanumeric;
    let chars: String = iter::repeat(())
        .map(|()| rng.sample(distr))
        .take(seed.len())
        .collect();
    return chars;
}

fn create_dir(mut dst: PathBuf, name: &str) -> Result<PathBuf, io::Error> {
    dst.push(name);
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
    return Ok(dst);
}

// Create the target directory where the files will be copied.
fn create_target_dir(home: &str) -> Result<PathBuf, io::Error> {
    create_dir([home, BASE_URL].iter().collect(), COPY_DEST)
}

// Return the file name for the backup archive.
fn create_archive_name(home: &str) -> Result<PathBuf, bool> {
    let mut fln: PathBuf = [home, BASE_URL, ARCHIVE_NAME].iter().collect();
    if fln.set_extension("tar") {
        return Ok(fln);
    }
    Err(false)
}

fn maybe_expand_home(f: &PathBuf) -> PathBuf {
    match f.to_owned().strip_prefix("~") {
        Ok(p) => Path::new(&home_name()).join(p),
        Err(e) => {
            error!("cannot strip (or no need to) $HOME from {:?}: {:?}", f, e);
            PathBuf::from(f)
        }
    }
}

fn maybe_expand_dot(f: &PathBuf) -> PathBuf {
    debug!("expanding DOT for {:?}", f);
    if f.to_str().unwrap().starts_with(".") {
        PathBuf::from(format!("dot{}", f.display()))
    } else {
        PathBuf::from(f)
    }
}

pub fn run(cfg: config::Config) -> Result<bool, io::Error> {
    match cfg.files.len() {
        0 => info!("nothing to do"),
        c => {
            info!("taking {:?} entries", c);
            // check what's the $HOME path here.
            let home: String = home_name();
            debug!("considering {} as $HOME", home);

            let dst: PathBuf = create_target_dir(&home).unwrap();
            // pick up one entry at the time from the given config.
            for cf in cfg.files {
                let mut f: PathBuf = PathBuf::from(cf);
                debug!("considering {:?}", f);

                f = maybe_expand_home(&f);

                match fs::metadata(&f) {
                    Ok(meta) => {
                        if meta.is_dir() {
                            debug!("{:?} is a directory", f);

                            info!(
                                "copying content of {:?} (file name: {:?} expanded as {:?}) into {:?}",
                                &f,
                                &f.file_name().unwrap(),
                                maybe_expand_dot(&PathBuf::from(&f.file_name().unwrap())),
                                &dst
                            );

                            let inner: PathBuf = create_dir(
                                dst.to_owned(),
                                &maybe_expand_dot(&PathBuf::from(&f.file_name().unwrap()))
                                    .into_os_string()
                                    .into_string()
                                    .unwrap(),
                            )?;

                            walkers::visit_dirs(&f, &|f_src| {
                                debug!("entering {:?} found {:?}", f, f_src.file_name());

                                let home = PathBuf::from(&home);

                                let f_dst: PathBuf =
                                    tag_name(&home, &inner, &PathBuf::from(f_src.file_name()));
                                debug!("destination filename: {:?}", &f_dst);
                                let f_src_s: PathBuf = [&f, &f_src.path()].iter().collect();
                                debug!("source filename: {:?}", &f_src_s);
                                return copy_file_in(f_src_s, f_dst);
                            })?;
                        } else {
                            debug!("{:?} is a file", f);
                            let f_dst: PathBuf = tag_name(
                                &PathBuf::from(&home),
                                &dst,
                                &maybe_expand_dot(&PathBuf::from(f.file_name().unwrap())),
                            );
                            debug!("destination filename: {:?}", &f_dst);
                            copy_file_in(f, f_dst)?;
                        }
                    }
                    Err(e) => error!("cannot read {:?}: {:?}", &f, e.description()),
                }
            }

            if let Ok(tan) = create_archive_name(&home) {
                info!("compressing {:?}", dst);
                compress::compress(&dst, &tan)?;

                info!("ciphering {:?}", tan);
                match encrypter::cipher(&tan) {
                    Ok(r) => info!("encryption of {:?} was successful ({:?})", &tan, r),
                    Err(e) => error!("error encrypting {:?}: {}", &tan, e),
                }
            } else {
                error!("cannot create the archive destination");
            };
        }
    };
    
    Ok(true)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
