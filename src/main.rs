use std::path::Path;
mod dir;

fn main() {
   let path = Path::new("../../../rust_wars/");
   let query = String::from("let delta_x");
   dir::visit_dirs(&path, &query); 
}
