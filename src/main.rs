use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;

use begaze::*;

////////////////////////////////////////////////////////////////////////////////


// Accompaying function to show directory item
// should be used with `visit_dirs` function
fn showEntry(cb: &fs::DirEntry) -> () {
    println!("  {:?}", cb.path());
}

// This function lists directory from provided path and
// applied provided function to it sequentially
fn visit_dirs(dir: &Path, cb: &Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

fn main() {
    let path =
        std::env::args()
          .nth(1)
          .expect("Argument 1 needs to be a path");

    println!("watching \"{}\":", path);

    visit_dirs(Path::new(&path), &showEntry);

}
