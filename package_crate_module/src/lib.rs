mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    pub mod serving {
        fn take_order() {}
        pub fn serve_order() {}
        pub fn take_payment() {}
    }
}

mod back_of_house {
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
}

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist(); // error, module `hosting` is private, function `add_to_waitlist` is private
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // meal.seasonal_fruit = String::from("blueberries"); // field `seasonal_fruit` of struct `Breakfast` is private
}

// use绝对路径和相对路径
// use crate::front_of_house::serving
use front_of_house::serving;
// fn:引用到要使用的fn的父级所在模块
// struct, enum:引用到自身完整路径，同名条目指定到父级，as指定本地别名
use back_of_house::Appetizer;
use back_of_house::Breakfast;

use std::fmt::Result;
use std::io::Result as IOResult;

// pub use
pub use back_of_house;

// 嵌套use
// use std::cmp::Ordering;
// use std::io;
use std::{cmp::Ordering, io};

// use std::io;
// use std::io::Write;
use std::io{self, Write};

// 使用通配符*导入所有公共模块，常用于tests模块或preload模块
use std::io::*;

pub fn pay() {
    serving::take_payment();
}
