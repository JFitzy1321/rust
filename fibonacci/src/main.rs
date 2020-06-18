use std::io::{stdin, stdout, Write};

fn main() {
    print!("Please enter a positive whole number: ");
    let _ = stdout().flush();

    let mut input_num = String::new();
    if let Err(_) = stdin().read_line(&mut input_num) {
        eprintln!("error: Something went wrong parsing your input!");
        std::process::exit(1);
    }

    let input_num = match input_num.trim().parse::<u32>() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("error: Please enter a positive whole number!");
            std::process::exit(1);
        }
    };

    println!(
        "The {} number of the Fibonacci Sequence is: {}",
        input_num,
        fib(&input_num)
    );
}

fn fib(input: &u32) -> u32 {
    let num = *input;
    // Get the value of input by dereference
    if num == 0 || num == 1 {
        num
    } else {
        // Need to make nth a reference again after calculation
        fib(&(num - 1)) + fib(&(num - 2))
    }
}
