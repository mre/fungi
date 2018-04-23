extern crate tar;

// use std::io::prelude::*;
use self::tar::Builder;
use std::fs::metadata;
use std::fs::File;
use std::io;
use std::path::PathBuf;

pub fn compress(src: &PathBuf, dst: &str) -> io::Result<()> {
    debug!("creating {:?}", dst);
    let file = File::create(dst).unwrap();
    debug!("building...");
    let mut a = Builder::new(file);

    match metadata(src) {
        Ok(meta) => {
            if meta.is_dir() {
                debug!("compressing {:?} as a directory", src.file_name().unwrap());
                a.append_dir_all(src.file_name().unwrap() , src).unwrap();
            } else {
                a.append_path(src.file_name().unwrap()).unwrap();
            }
        }
        _ => panic!("cannot read the path to compress"),
    }
    a.finish()
}
