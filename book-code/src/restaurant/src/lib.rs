mod front_of_house;

pub use crate::back_of_house::{Breakfast, Toast};
pub use crate::front_of_house::hosting;

pub mod back_of_house {
    #[derive(Debug)]
    pub enum Toast {
        Rye,
        Wheat,
        White,
        Sourdough,
        None,
    }

    pub struct Breakfast {
        pub toast: Toast,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: Toast) -> Self {
            Self {
                toast,
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // // Absolute path
    // crate::front_of_house::hosting::add_to_waitlist();

    // // Relative Path
    // front_of_house::hosting::add_to_waitlist();

    hosting::add_to_waitlist();
    hosting::seat_at_table();

    let mut meal = Breakfast::summer(Toast::Rye);
    meal.toast = Toast::Wheat;
    println!("I'd like {:?} toast please", meal.toast);
}
#[cfg(test)]
mod tests {
    #[test]
    fn eat_at_restaurant_works() {
        super::eat_at_restaurant();
    }
}
