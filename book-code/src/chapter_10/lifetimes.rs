// // Code doesn't compile because x is dropped,
// // but r still has a reference to it
// pub fn main() {
//     let r;
//     {
//         let x = 5;
//         r = &x;
//     }

//     println!("r: {}", r);
// }

// pub fn main() {
//     let string1 = "abcd".to_string();
//     let string2 = "xyz";

//     println!(
//         "The longest string is '{}'",
//         longest(string1.as_str(), string2)
//     );
// }

// // Code won't compile because result code have reference to string2,
// // which code be referenced outside it's lifetime (scope in this case)
// pub fn main() {
//     let string1 = "abcd".to_string();
//     let result;
//     {
//         let string2 = "xyz".to_string();
//         result = longest(string1.as_str(), string2.as_str());
//     }
//     println!("The longest string is '{}'", result);
// }
#[allow(dead_code)]
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[allow(dead_code)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

#[allow(dead_code)]
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

pub fn main() {
    let novel = "Call me Ishmael. Some years ago...";
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let _i = ImportantExcerpt {
        part: first_sentence,
    };
}
