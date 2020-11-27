use std::io;

pub fn main() {
    println!("Number Guessing Game!");

    println!("Please enter a number.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to readline!");

    println!("You guessed: {}", guess);
}
