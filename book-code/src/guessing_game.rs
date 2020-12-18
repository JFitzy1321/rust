use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn main() {
    println!("Number Guessing Game!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please enter a number.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let guess = guess.trim();

        if guess.trim().eq("show") {
            println!("The secret number is: {}", secret_number);
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(t) => t,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
