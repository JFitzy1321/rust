extern crate rand; // lets my code know about the crates it can access

// import things i need
use core::num::ParseIntError;
use rand::Rng;
use std::cmp::Ordering;
use std::io::{stdin, stdout, Write};

pub fn main() {
    println!("Number Guessing Game written in Rust!!\n");

    let rand_num: u32 = rand::thread_rng().gen_range(1, 101);

    loop {
        let guess: u32 = match get_and_parse_input(&rand_num) {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter only whole numbers!");
                continue;
            }
        };

        print!("You guessed {}. ", guess);
        match guess.cmp(&rand_num) {
            Ordering::Less => println!("Number too small!\n"),
            Ordering::Greater => println!("Number to large\n"),
            Ordering::Equal => {
                println!("You win!\n");
                break;
            }
        };
    }
}

fn get_and_parse_input(secret_num: &u32) -> Result<u32, ParseIntError> {
    loop {
        print!("Enter a number between 1 and 100: ");
        let _ = stdout().flush(); // flush output to print and read from single line

        let mut guess = String::new();

        match stdin().read_line(&mut guess) {
            Ok(_) => {
                if guess.contains("printnum") {
                    println!("{}", secret_num);
                    continue;
                }
            }
            Err(_) => {
                println!("Something went wrong reading your answer. Try again or ctrl+c to crash the game");
                continue;
            }
        };

        return guess.trim().parse::<u32>();
    }
}
