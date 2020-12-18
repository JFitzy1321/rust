use std::error::Error;
use std::fs;
use std::process;

struct Config<'a> {
    query: &'a str,
    filename: &'a str,
}

impl<'a> Config<'a> {
    fn new(args: &'a [String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments passed in");
        }

        Ok(Self {
            query: args[1].as_str(),
            filename: args[2].as_str(),
        })
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let _ = run(config);
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    println!("With text:\n{}", contents);
    Ok(())
}

#[cfg(test)]
mod bin_tests {
    use std::vec;

    use super::*;
    #[test]
    fn config_new_panics_with_1_arg() {
        let args = vec!["Hello Rust Tests".to_string()];
        let result = Config::new(&args);
        assert!(result.is_err());
    }

    #[test]
    fn config_new_panics_with_2_arg() {
        let args = vec!["Hello Rust".to_string(), "Tests".to_string()];
        let result = Config::new(&args);
        assert!(result.is_err());
    }

    #[test]
    fn config_new_works_with_3_args() {
        let query = "Rust";
        let filename = "Tests";
        let args = vec!["Hello".to_string(), query.to_string(), filename.to_string()];
        let config = Config::new(&args).unwrap();
        assert_eq!(query, config.query);
        assert_eq!(filename, config.filename);
    }

    #[test]
    fn config_new_works_with_multiple_args() {
        let query = "b";
        let filename = "c";
        let args = vec![
            "a".to_string(),
            query.to_string(),
            filename.to_string(),
            "d".to_string(),
            "e".to_string(),
            "f".to_string(),
        ];
        let config = Config::new(&args).unwrap();
        assert_eq!(query, config.query);
        assert_eq!(filename, config.filename);
    }
}
