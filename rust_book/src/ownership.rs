// pub fn main(){
//     let s = String::from("hello");

//     takes_ownership(s);

//     let x = 5;

//     makes_copy(x);
// }

// fn takes_ownership(some_string: String) {
//     println!("{}", some_string);
// }

// fn makes_copy(some_integer: i32) {
//     println!("{}", some_integer);
// }

// pub fn main() {
//     let _s1 = gives_ownership();

//     let s2 = String::from("hello");

//     let _s3 = takes_and_gives_back(s2);
// }

// fn gives_ownership() -> String {
//     String::from("hello")
// }

// fn takes_and_gives_back(a_string: String) -> String {
//     a_string
// }

// pub fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1);

//     println!("The length of '{}' is {}", s1, len);
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// pub fn main() {
//     let mut s = String::from("hello");

//     change(&mut s);
// }
// // fn change(some_string: &String) {} // this is a 'borrowed' reference
// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

pub fn main() {
    let mut s = "hello".to_string();

    let r1 = &s;
    let r2 = &s;

    println!("{} and {}", r1, r2);

    let r3 = &mut s;
    println!("{}", r3);
}