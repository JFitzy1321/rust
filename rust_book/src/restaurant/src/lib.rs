/// Path to Material: file:///C:/Users/joe.fitzgibbons/.rustup/toolchains/stable-x86_64-pc-windows-msvc/share/doc/rust/html/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html
mod front_of_house;

pub use crate::front_of_house::hosting;

fn serve_order() {
    front_of_house::serving::serve_order();
}

mod back_of_house {
    // enum variants are all public if enum is marked public
    pub enum Appetizer {
        Soup,
        Salad,
    }

    // Must make fields public in struct to access outside of the struct
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}

use crate::back_of_house::Appetizer;

pub fn eat_at_restaurant() {
    // // Absolute path from "root" (crate is root of current package)
    // crate::front_of_house::hosting::add_to_waitlist();

    // // Relative path
    // front_of_house::hosting::add_to_waitlist();

    //     let mut meal = back_of_house::Breakfast::summer("Rye");
    //     meal.toast = "Wheat".to_string();
    //     println!("I'd like {} toast please", meal.toast);
    //     // meal.seasonal_fruit = "Strawberries".to_string();

    let order1 = Appetizer::Soup;
    let order2 = Appetizer::Salad;
}
