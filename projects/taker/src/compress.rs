extern crate tar;

// use std::io::prelude::*;
use std::fs::File;
use std::path::PathBuf;
use tar::Builder;

mod walkers;

fn compress(src: PathBuf, dst: &str) {
    let file = File::create(dst).unwrap();
    let mut a = Builder::new(file);

    visit_dirs(&f, &|f_src| {
        debug!("entering {:?} found {:?}", f, f_src.file_name());
    });

    a.append_path("file1.txt").unwrap();
    a.append_file("file2.txt", &mut File::open("file3.txt").unwrap())
        .unwrap();
}
