extern crate tar;

// use std::io::prelude::*;
use self::tar::Builder;
use std::fs::File;
use std::path::PathBuf;
use std::fs::DirEntry;
use std::io;

use walkers;

pub fn compress(src: &PathBuf, dst: &str) -> io::Result<()> {
    let file = File::create(dst).unwrap();
    let mut a = Builder::new(file);

    return walkers::visit_mut_dirs(src, &mut move |f_src: &DirEntry| {
        debug!("entering {:?} found {:?}", src, f_src.file_name());

        a.append_path("file1.txt").unwrap();
        a.append_file("file2.txt", &mut File::open("file3.txt").unwrap())
            .unwrap();

        return Ok(true);
    });
}
