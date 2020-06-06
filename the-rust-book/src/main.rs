fn main() {
//     println!("Hello, cargo! Again...");
//     chapter_2_game::main();
//     chapter_3::main();
    chapter_4::main();
}

// /// Chapter 2: Guessing Game
// mod chapter_2_game {
//     use rand::Rng;
//     use std::cmp::Ordering;
//     use std::io::Write;

//     pub fn main() {
//         println!("Guess the number!");
//         let secret_num: u32 = rand::thread_rng().gen_range(1, 101);
//         println!("The secret number is: {}", secret_num);
//         loop {
//             print("Please input your guess: ");
//             let mut guess = String::new();
//             std::io::stdin()
//                 .read_line(&mut guess)
//                 .expect("Failed to read line");
//             let guess = guess.trim();
//             if ["quit", "exit"].contains(&guess.to_lowercase().as_ref()) {
//                 println!("Quitting game!");
//                 break;
//             }
//             let guess: u32 = match guess.parse() {
//                 Ok(num) => num,
//                 Err(_) => {
//                     println!("Please enter a whole number!");
//                     continue;
//                 }
//             };
//             println!("You guessed {}", guess);
//             match guSess.cmp(&secret_num) {
//                 Ordering::Less => println!("Too small!"),
//                 Ordering::Greater => println!("Too big!"),
//                 Ordering::Equal => {
//                     println!("You win!");
//                     break;
//                 }
//             }
//         }
//     }

//     fn print(message: &str) {
//         print!("{}", message);
//         std::io::stdout().flush().unwrap();
//     }
// }

// /// Chapter 3: Common Programming Concepts
// mod chapter_3 {
//     pub fn main() {
//         println!("Chapter 3: Variables!");
//         variables();

//         println!("Chapter 3: Shadowing!");
//         shadowing();

//         println!("Chapter 3: Array indexing");
//         out_of_bounds_array();
//         let num = 43;
//         let num_type = ternary_num_type(&num);
//         println!("Number is {}", num_type);
//         for_loop_iterator();
//         for_range();
//     }
//     pub fn variables() {
//         let x = 5;
//         println!("X is: {}", x);
//         let x = 6;
//         println!("X is now {} after shadowing!", x);
//         const MAX_INT: u32 = 100_000;
//     }

//     pub fn shadowing() {
//         let text = "I am some text";
//         println!("{}", text);
//         let text = 32_876;
//         println!("{}", text);
//     }

//     pub fn out_of_bounds_array() {
//         let num_array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
//         let index = 20;

//         println!("The {} element is: {}", index, num_array[index]);
//     }

//     pub fn ternary_num_type<'a>(num: &'a u32) -> &'a str {
//         // how todo a ternary operator in rust
//         if num % 2 == 0 {
//             "even"
//         } else {
//             "odd "
//         }
//     }

//     fn for_loop_iterator() {
//         let a = [10, 20, 30, 40, 50];

//         for elem in a.iter() {
//             println!("{}", elem);
//         }
//     }

//     fn for_range() {
//         for num in (1..5).rev() {
//             println!("{}!", num);
//         }
//         println!("BLAST OFF!!!");
//     }
// }
/// Chapter 4: Understanding Ownership
mod chapter_4 {
    pub fn main() {
        let something = String::from("Hello Stack Allocated Types");
    }
}
