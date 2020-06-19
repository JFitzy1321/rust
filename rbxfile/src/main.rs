/// Creating a cli in Rust: https://docs.rs/structopt/0.3.7/structopt/
///  TODO: ^^^^ read this doc
///
/// This tool will create new rust files.
/// Needs one argument, the name of the file to create.
///
/// Files will be written with an empty main function
/// and a comment saying "// Path to Material: [ path or url here ]".
///
/// ## expected output
/// ```rust
/// // Path to Material: [ path or url here ]
///
/// pub fn main() {
///
/// }
/// ```
//TODO: learn how doctests work
//TODO: learn rust documentation style and implement it
#[macro_use]
extern crate clap;

mod file_io {
    use std::fs::OpenOptions;
    use std::io::{self, Write};
    use std::path::Path;

    pub fn create_rbx_file(filename: &str) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(Path::new(filename))?;

        file.write_all(b"/// Path to Material: [url or path here]\n\npub fn main() {\n\n}")
    }
}

mod args {
    // https://github.com/clap-rs/clap
    pub fn get_filename() -> String {
        let matches = app_from_crate!()
            .arg(clap::Arg::with_name("filename").required(true))
            .get_matches();

        //clap documentation says its okay to use unwrap on required arguments
        matches.value_of("filename").unwrap().to_string()
    }
}

fn main() -> Result<(), std::io::Error> {
    let _arg = args::get_filename();
    let fname = if !_arg.ends_with(".rs") {
        format!("{}{}", _arg, ".rs")
    } else {
        _arg
    };

    match file_io::create_rbx_file(&fname) {
        Ok(()) => {
            println!(
                "File {} created in folder {} successfully!",
                &fname,
                std::env::current_dir().unwrap().display()
            );
            Ok(())
        }
        Err(e) => Err(e),
    }
}
