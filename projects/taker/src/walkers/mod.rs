use std::path::Path;
use std::fs::{self, DirEntry, ReadDir};
use std::io;

// https://doc.rust-lang.org/std/fs/fn.read_dir.html
// one possible implementation of walking a directory only visiting files
pub fn visit_dirs<T>(dir: &Path, cb: &T) -> io::Result<()>
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

#[allow(dead_code)]
pub fn visit_mut_dirs<T>(dir: &Path, cb: &mut T) -> io::Result<()>
where
    T: FnMut(&DirEntry) -> Result<bool, io::Error>,
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
                        visit_mut_dirs(&path, cb)?;
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
