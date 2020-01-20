/// Path to Material: https://doc.rust-lang.org/book/ch03-05-control-flow.html
// use std::io;

// pub fn main() {
//     //i already know how if statments work,
//     //does rust support ternary operations?

//     //inifinite loop
//     loop {
//         let mut input = String::new();

//         println!("Enter a number to see if it's even or odd.");
//         match io::stdin().read_line(&mut input) {
//             Ok(_n) => {}
//             Err(_e) => println!("Looping again until you enter a number"),
//         };

//         let input: i32 = input
//             .trim()
//             .parse()
//             .expect("Did you not enter a number, you fucking illiterate moron?");

//         //rust does not currently have a ternary operator, but instead uses python like ternary
//         //remember, if the line at the end of scope doesn't have a semicolon, than it's an
//         //expression, which results in a value
//         let output = if input % 2 == 0 { "Even" } else { "Odd" };

//         println!("\nYou entered: {}", input);
//         println!("Your number is: {}", output);
//     }
// }

// pub fn iterate_array() {
//     let array: [i32; 5] = [1, 2, 3, 4, 5];

//     println!("looping through iterates");
//     for num in array.iter() {
//         println!("num: {}", num);
//     }

//     println!("looping through range based for loop");
//     for i in 0..5 {
//         println!("index: {}  num: {}", i, array[i]);
//     }
// }
pub fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 20 {
            break counter;
        }
    };

    println!("Number of loops: {}", result);
}
