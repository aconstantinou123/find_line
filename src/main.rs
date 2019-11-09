mod dir;

use std::path::Path;
use dir::Config;
use std::{env, process};

fn main() {
   let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    
    let path = Path::new(&config.path);
    if let Err(e) = dir::visit_dirs(&path, &config.query) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
  
}
