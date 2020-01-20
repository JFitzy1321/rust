// Path to Material: https://doc.rust-lang.org/book/ch06-02-match.html

pub fn main() {
    let plus_one = |x: Option<i32>| -> Option<i32> {
        match x {
            Some(i) => Some(i + 1),
            None => None,
        }
    };

    let five = Some(5);
    let six = plus_one(five);
    println!("after plus one: {:?}", six);

    let none = plus_one(None);
    println!("plus one one None variant: {:?}", none);
}
