mod chapter_2;
mod chapter_3;
mod chapter_4;
mod chapter_5;
mod chapter_6;
mod katas;

// mod chapter_6;
// use chapter_6::{Coin, UsState};

fn main() {
    // let quarter = Coin::Quarter(UsState::NewYork);
    // match quarter {
    //     Coin::Quarter(state) => println!(" I have a quarter from {:?}", state),
    //     _ => println!("I have a {} cent coin!", quarter.value()),
    // }

    chapter_2::guessing_game::main();
    let _ = katas::descending_ints::main(123);
}
