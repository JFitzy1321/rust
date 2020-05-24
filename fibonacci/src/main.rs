use std::io::{stdin, stdout, Write};

fn main() {
    print!("Please enter a positive whole number: ");
    let _ = stdout().flush();

    let mut input_num = String::new();
    stdin()
        .read_line(&mut input_num)
        .expect("Something went wrong reading your input!");
    let input_num: u32 = input_num
        .trim()
        .parse()
        .expect("Please enter a positive whole number!");

    println!(
        "The {} number of the Fibonacci Sequence is: {}",
        input_num,
        fib(&input_num)
    );
}

fn fib(input: &u32) -> u32 {
    // Get the value of input by dereference
    if *input == 0 || *input == 1 {
        *input
    } else {
        // Need to make nth a reference again after calculation
        fib(&(*input - 1)) + fib(&(*input - 2))
    }
}
