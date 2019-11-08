use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;

pub fn visit_dirs(dir: &Path) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            println!("path {:?}", path);
            if path.is_dir() {
                visit_dirs(&path)?;
            } else {
            println!("finish");
            }
        }
    }
    Ok(())
}
