use std::fs::File;
use std::io::prelude::*;

pub fn new_file(name: &str) -> std::io::Result<()> {
    let mut path = String::from(name);

    if !name.ends_with(".rs") {
        path.push_str(".rs");
    }

    let mut file = File::create(Path::new(&path))?;
    file.write_all(b"// Path or Material: [url or path here]\n\npub fn main() {\n\n}")?;
}
