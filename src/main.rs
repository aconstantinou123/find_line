use std::path::Path;
mod dir;

fn main() {
   let path = Path::new("../../../rust_wars/");
   let query = String::from("struct");
   dir::visit_dirs(&path, &query); 
}
