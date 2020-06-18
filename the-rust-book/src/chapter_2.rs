/// Chapter 2: Guessing Game
use rand::Rng;
use std::cmp::Ordering;
use std::io::Write;

pub fn main() {
    println!("Guess the number!");
    let secret_num: u32 = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_num);
    loop {
        print("Please input your guess: ");
        let mut guess = String::new();
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess = guess.trim();
        if ["quit", "exit"].contains(&guess.to_lowercase().as_ref()) {
            println!("Quitting game!");
            break;
        }
        let guess: u32 = match guess.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a whole number!");
                continue;
            }
        };
        println!("You guessed {}", guess);
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn print(message: &str) {
    print!("{}", message);
    std::io::stdout().flush().unwrap();
}
