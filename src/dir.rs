use std::io;
use std::fs::{self, DirEntry, File};
use std::path::Path;
use std::io::Read;
use std::str;

pub fn visit_dirs(dir: &Path, query: &str) -> io::Result<()> {
    if dir.is_dir() {
    let mut path;
    let mut dirs = vec![];
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            path = entry.path();
            if path.is_dir() {
                dirs.push(path);
            } else if path.is_file() {
                let contents = match fs::read_to_string(&path) {
                    Ok(f) => f,
                    Err(e) => e.to_string(),
                };

                search(&query, &contents);
            }
        }
        for dir_path in dirs.iter(){
            println!("dir: {:?}", dir_path);
            visit_dirs(&dir_path, &query)?;
        }

    }
    Ok(())
}

fn read_file(path: &Path) -> Result<String, ()> {
    let contents = fs::read_to_string(path)
        .expect("fail");
    Ok(contents)
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let filtered = contents.lines()
        .filter(|line| line.contains(query))
        .collect();
    println!("filtered: {:?}", filtered);
    filtered
}
