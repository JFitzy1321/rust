// Path to Material: file:///<HOME DIR>/.rustup/toolchains/stable-x86_64-pc-windows-msvc/share/doc/rust/html/book/ch06-03-if-let.html

use std::io::{stdin, stdout, Write};

pub fn main() {
    let input: Option<i32> = get_input();

    if let None = input {
        println!("Please enter a number next time, idiot!");
        return;
    }

    println!("Your number: {}", input.unwrap());
}

fn get_input() -> Option<i32> {
    print!("Enter a number between 1 and 100: ");
    stdout().flush().unwrap();

    let mut user_input = String::new();

    stdin().read_line(&mut user_input).unwrap();

    match user_input.trim().parse::<i32>() {
        Ok(num) => Some(num),
        Err(_) => None,
    }
}
