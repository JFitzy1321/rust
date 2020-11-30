
pub fn add_to_waitlist() {}
pub fn seat_at_table() {}

pub mod serving {
    fn take_order() {}

    fn serve_order() {}

    fn take_payment() {}

    mod back_of_house {
        fn cook_order() {}

        fn fix_incorrect_order() {
            cook_order();
            super::serve_order();
        }
    }
}
