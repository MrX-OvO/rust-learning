#[derive(Debug)]
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    fn new() -> AveragedCollection {
        AveragedCollection {
            list: vec![],
            average: 0.0,
        }
    }

    fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    fn remove(&mut self) -> Option<i32> {
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

    fn average(&self) -> f64 {
        self.average
    }

    fn list(&self) -> &[i32] {
        &self.list[..]
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
