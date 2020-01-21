use std::path::Path;
use std::fs::File;
use std::io::Write;

pub fn main(name: &str) -> std::io::Result<()> {
    let mut path = String::from(name);
    if !name.ends_with(".rs") {
        path.push_str(".rs");
    }

    let mut file = File::create(Path::new(&path))?;

    file.write(b"// Path to Rust Section: [enter here]\n\npub fn main() {\n\n}")?;
    Ok(())
}