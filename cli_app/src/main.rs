/// Path to Material: https://rust-cli.github.io/book/tutorial/output.html

// use structopt::StructOpt;

// #[derive(StructOpt)]
// struct Cli {
//     pattern: String,
//     #[structopt(parse(from_os_str))]
//     path: std::path::PathBuf,
// }

// #[derive(Debug)]
// struct CustomError(String);

// fn main() -> Result<(), CustomError> {
//     // let args = Cli::from_args();

//     // let content = std::fs::read_to_string(&args.path)?;

//     // for line in content.lines() {
//     //     if line.contains(&args.pattern) {
//     //         println!("{}", line);
//     //     }
//     // }

//     let path = "test.txt";

//     let content = std::fs::read_to_string(path)
//         .map_err(|err| CustomError(format!("Error reading `{}` : {}", path, err)))?;
//     println!("file content: {}", content);
//     Ok(())
// }

// use exitfailure::ExitFailure;
// use failure::ResultExt;

// fn main() -> Result<(), ExitFailure> {
//     let path = "test.txt";
//     let content = std::fs::read_to_string(path)
//         .with_context(|_| format!("could not read file `{}`", path))?;
//     println!("file content: {}", content);
//     Ok(())
// }

/// Progress Bar
use indicatif;

fn main() {
    let pb = indicatif::ProgressBar::new(100);
    for _ in 0..100 {
        do_hard_work();
        //pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }
    pb.finish_with_message("done!");
}

fn do_hard_work() {
    use std::{thread, time};

    thread::sleep(time::Duration::from_millis(500));
}
