use std::path::Path;
use find_words::Config;
use std::{env, process};

fn main() {
   let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    
    let path = Path::new(&config.path);
    if let Err(e) = find_words::visit_dirs(&path, &config.query) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
  
}
