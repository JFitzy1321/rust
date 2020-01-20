use std::io::{stdin, stdout, Write};

pub fn main()
{
    print!("Enter some text: ");
    let _ = stdout().flush().expect("Problem flushing stdout");

    let mut input = String::new();
    stdin().read_line(&mut input).expect("Problem reading text");

    println!("This word of your text is '{}'", first_word(&input));

//     let my_string = String::from("Hello world");

//     //first_word works on slices of strings
//     let word = first_word(&my_string[..]);

//     let str_literal = "fuck world";
//     // Because string literals *Are* string slices already,
//     // this works too, without the slice syntax!
//     let word = first_word(str_literal);
}

fn first_word(sentence: &str) -> &str {
    for (i, &item) in sentence.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &sentence[0..i];
        }
    }
    &sentence[..]
}

