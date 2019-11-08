use std::path::Path;
mod dir;

fn main() {
   let path = Path::new("../../../../projects"); 
   dir::visit_dirs(&path); 
}
