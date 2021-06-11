use std::{
    io::{stdin, stdout, Write},
    process::exit,
    thread::sleep,
    time::Duration,
};

pub fn main() {
    println!("Fibonacci Sequence Calculator, written in Rust!");
    println!("This app will give you the fibonacci number at a given point in the sequence.");

    let valid_continues: Vec<&str> = vec!["y", "Y", "yes", "Yes"];

    loop {
        sleep(Duration::from_millis(500));

        // using print! and flush to have input on the same line as output
        // I think this looks better, but it doesn't really matter
        print!("\nPlease enter a positive whole number: ");
        let _ = stdout().flush();

        let mut input_num = String::new();
        stdin()
            .read_line(&mut input_num)
            .expect("error: Something went wrong parsing your input!");

        let input_num: u32 = input_num
            .trim()
            .parse()
            .expect("error: Please enter a positive whole number!");

        println!(
            "The {} number of the Fibonacci Sequence is: {}",
            input_num,
            fib(&input_num)
        );

        print!("\nWould you like to calculate another number? (y/n) : ");
        let _ = stdout().flush();

        let mut continue_input = String::new();
        let _ = stdin()
            .read_line(&mut continue_input)
            .expect("error: Something went wrong reading your answer!");

        if !(valid_continues.contains(&continue_input.trim())) {
            println!("Goodbye");
            exit(0);
        }
    }
}

// TODO: How can I calculate the actual sequence?

fn fib(input: &u32) -> u32 {
    if *input == 0 || *input == 1 {
        *input
    } else {
        // Need to make nth number a reference again after calculation
        fib(&(*input - 1)) + fib(&(*input - 2))
    }
}
