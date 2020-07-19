// mod chapter_1;
// mod chapter_2;
// mod chapter_3;
//mod chapter_4;
// mod chapter_5;
// mod chapter_6;
// use crate::chapter_6::Coin::{self, Dime, Nickel, Penny, Quarter};
// fn main() {
//     // println!("Hello, cargo! Again...");
//     // chapter_2::main();
//     // chapter_3::main();
//     // chapter_4::main();
//     // chapter_5::main();
//     chapter_6::main();
//     let dime: Coin = Dime;
// }

mod chapter_6;
use chapter_6::{Coin, UsState};

fn main() {
    let quarter = Coin::Quarter(UsState::NewYork);
    match quarter {
        Coin::Quarter(state) => println!(" I have a quarter from {:?}", state),
        _ => println!("I have a {} cent coin!", quarter.value()),
    }
}
