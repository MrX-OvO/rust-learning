fn fix_incorrect_order() {
    cook_order();
    super::front_of_house::serving::serve_order();
}
fn cook_order() {}

pub struct Breakfast {
    // struct的字段默认私有，通过设置pub关键字变为共有
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

pub enum Appetizer {
    // 都是公共的，包括enum和enum的变体
    Soup,
    Salad,
}
