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
    let mut s = String::new();
    match first_word(&s) {
        Some(s) => println!("The first word in the string is: {}", s),
        None => println!("No word found in string!"),
    };

    s.clear();
}
fn first_word(s: &String) -> Option<&str> {
    if let Some(s) = s.trim().split_whitespace().collect::<Vec<&str>>().first() {
        Some(s)
    } else {
        None
    }
}
