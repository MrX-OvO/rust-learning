mod back_of_house;
mod front_of_house;

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist(); // error, module `hosting` is private, function `add_to_waitlist` is private
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // meal.seasonal_fruit = String::from("blueberries"); // field `seasonal_fruit` of struct `Breakfast` is private
}
