extern crate tar;

// use std::io::prelude::*;
use self::tar::Builder;
use std::fs::metadata;
use std::fs::{DirEntry, File};
use std::io;
use std::path::PathBuf;

use walkers;

pub fn compress(src: &PathBuf, dst: &str) -> io::Result<()> {
    let file = File::create(dst).unwrap();
    let mut a = Builder::new(file);

    return walkers::visit_mut_dirs(src, &mut move |f: &DirEntry| {
        debug!("entering {:?} found {:?}", src, f.file_name());

        match metadata(PathBuf::from(f.file_name())) {
            Ok(meta) => {
                if meta.is_dir() {
                    // TODO: fix duplication here (visit should stop the
                    // traversal.
                    debug!("{:?} is a directory", f);
                    a.append_dir_all(f.file_name(), f.file_name()).unwrap();
                } else {
                    a.append_path(f.file_name()).unwrap();
                }
            }
            _ => panic!("boom"),
        }
        return Ok(true);
    });
}
