use std::io;
use std::{ fs, env };
use std::path::Path;
use std::str;
use termion;

pub struct Config {
    pub query: String,
    pub path: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("No query entered"),
        };

        let path = match args.next() {
            Some(arg) => arg,
            None => return Err("No filename entered"),
        };

        let case_sensitive = match args.next() {
            Some(arg) => if arg == "i" { false } else { return Err("invalid arguments") },
            None => env::var("CASE_INSENSITIVE").is_err(),
        };

        Ok(Config { query, path, case_sensitive })
    }
}

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
                    Err(e) => {
                        e.to_string()
                    },
                };
                if contents != "stream did not contain valid UTF-8" {
                    search(&query, &contents, &path);
                }
            }
        }
        for dir_path in dirs.iter(){
            visit_dirs(&dir_path, &query)?;
        }

    }
    Ok(())
}


fn search(query: &str, contents: &str, path: &Path) -> Vec<String> {
    let mut counter = 1;
    let filtered: Vec<String> = contents.lines()
        .map(|line| {
            counter += 1;
            format!("{}: {}", counter, line.trim())
        })
        .filter(|line| line.contains(query))
        .collect();
    if filtered.is_empty() { 
        println!("file: {:?}", path);
        println!("{}{}{}", termion::cursor::Up(1), termion::clear::CurrentLine, termion::cursor::Up(1));
    } else {
         println!("\nfile: {:?}\n", path);
    }
    for line in filtered.iter() {
        println!("{}", line);
    }
    filtered
}
