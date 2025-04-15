mod front_of_house;

use std::fmt::Result;
use std::io::Result as IoResult;
use std::io::{self, Write};
pub use front_of_house::hosting;

// fn function1() -> Result {
//
// }
//
// fn function2() -> IoResult<()> {
//
// }

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }

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
        super::deliver_order();
    }

    fn cook_order() {}
}

mod customer {
    use crate::{back_of_house, front_of_house};
    use crate::front_of_house::hosting;
    use crate::front_of_house::hosting::add_to_waitlist;

    pub fn eat_at_restaurant() {
        // absolute path
        crate::front_of_house::hosting::add_to_waitlist();

        // relative path
        front_of_house::hosting::add_to_waitlist();

        // Order a breakfast in the summer with Rye toast
        let mut meal = back_of_house::Breakfast::summer("Rye");
        // Change our mind about what bread we'd like
        meal.toast = String::from("Wheat");
        println!("I'd like {} toast please", meal.toast);

        // The next line won't compile if we uncomment it; we're not
        // allowed to see or modify the seasonal fruit that comes
        // with the meal.
        // meal.seasonal_fruit = String::from("blueberries");

        let order1 = back_of_house::Appetizer::Soup;
        let order2 = back_of_house::Appetizer::Salad;

        hosting::add_to_waitlist();

        add_to_waitlist();
    }
}

fn deliver_order() {}