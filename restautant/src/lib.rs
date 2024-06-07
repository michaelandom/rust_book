
mod front_of_house;

fn serve_order() {}

mod back_of_house;

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();
    let mut meal = back_of_house::Breakfast::summer("as");
    meal.toast = String::from("ww");
    let order1= back_of_house::Appetizer::Soup;
    let order2= back_of_house::Appetizer::Salad;
    hosting::add_to_waitlist();
}