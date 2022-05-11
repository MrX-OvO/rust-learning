/* #[derive(Debug)]
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn list(&self) -> &[i32] {
        &self.list[..]
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    pub fn new() -> AveragedCollection {
        AveragedCollection {
            list: vec![],
            average: 0.0,
        }
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        if let Some(value) = result {
            self.update_average();
            Some(value)
        } else {
            None
        }
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        let len = self.list.len();
        self.average = total as f64 / len as f64;
    }
}

fn main() {
    let mut obj = AveragedCollection::new();
    println!("obj: {:?}", obj);

    for i in 0..10 {
        obj.add(i);
    }
    println!("after add\nobj: {:?}", obj);

    let result = obj.remove();
    println!("after remove\nresult: {:?}", result);
    println!("obj: {:?}", obj);
}
 */

use oop::Draw;
use oop::{Button, Screen};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("draw selecbox...");
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 32,
                height: 32,
                label: String::from("Ok"),
            }),
        ],
    };
    screen.run();
}
