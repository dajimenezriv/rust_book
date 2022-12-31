// by default, our crates are private
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

mod back_of_house {
    // if we use pub before a struct definition, we make the struct public, but the struct’s fields will still be private
    // but enum fields are public by default
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    // also, because Breakfast has a private field we can't create an instance from another class
    // we need a public function that returns an instance
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

// if we want to import multiple modules from the same path
// we can add pub so everything that imports this module can use also hosting and add_to_waitlist directly
pub use crate::front_of_house::hosting::{self, add_to_waitlist};

pub fn eat_at_restaurant() {
    // it will throw an error if addd_to_waitlist if private
    crate::front_of_house::hosting::add_to_waitlist(); // absolute path
    front_of_house::hosting::add_to_waitlist(); // relative path
    hosting::add_to_waitlist(); // accessing with use notation

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    // we can't modify the seasonal_fruit since its private
    // meal.seasonal_fruit = String::from("blueberries"); // ! error
    println!("I'd like {} toast please", meal.toast);
}
