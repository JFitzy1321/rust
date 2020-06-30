// Chapter 4: Understanding Ownership
pub fn main() {
    // let something = String::from("Hello Stack Allocated Types");
    // let a: Vec<f64> = (0..100).map(|n| n as f64).collect();
    // main_copy_code();
    // main_reference();
    main_str_slice();
}

// fn main_copy_code() {
//     let x = 42;

//     make_copy(x);

//     println!("Owned value {}", x);
//     // make_copy(String::from("Derp"));
// }

// /// Using Copy Trait to restrict Generic
// fn make_copy<T: Copy>(copy_t: T) where T: std::fmt::Display {
//     println!("Copied value: {}", copy_t);
// }

// fn main_reference() {
//     let s1 = String::from("Yet another Hello World");

//     println!("Length of this string: '{0}' : {1}", s1, calculate_length(&s1))
// }

// fn calculate_length(s: &str) -> usize {
//     s.len()
// }

fn main_str_slice() {
    use std::io::{stdin, stdout, Write};

    print!("Please enter a setence: ");
    let _ = stdout().flush();

    let mut s = String::new();
    if let Err(err) = stdin().read_line(&mut s) {
        eprintln!("Error: Something went wrong parsing text!\n{:?}", err);
        std::process::exit(1);
    }

    match first_word(&s) {
        Some(s) => println!("The first word in the string is: {}", s),
        None => println!("No word found in string!"),
    };
}

fn first_word(s: &str) -> Option<&str> {
    s.split_whitespace().next()
}
