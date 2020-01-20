// use std::io::{stdin, stdout, Write};

// pub fn main() {
//     println!("Calculating Fibonacci Numbers in rust");

//     let num = 0;

//     loop {
//         let mut nth = String::new();

//         print!("Enter a number to calculate fibonacci numbers up to: ");
//         stdout().flush();

//         match stdin().read_line(&mut nth) {
//             Ok(n) => n,
//             Err(_) => {
//                 println!("Something went wrong getting your input!");
//                 continue;
//             }
//         };
//         let nth: u32 = match nth.trim().parse() {
//             Ok(n) => n,
//             Err(_) => {
//                 println!("Please enter only whole numbers");
//                 continue;
//             }
//         };
//         fibonacci(nth);
//         break;
//     }
// }

// fn fibonacci(n: u32) {
//     let num = match n {
//         0 => 0,
//         1 => 1,
//         _ => {
//             let num = fibonacci(n - 1) + fibonacci(n - 2);
//         }
//     };
// }
