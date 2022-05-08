use std::{collections::HashMap, thread, time::Duration};

// 创建一个存放闭包和调用闭包结果的结构体
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    //value: Option<u32>,
    value: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        let v = self.value.entry(arg).or_insert((self.calculation)(arg));
        *v
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    // 保存一个新的 Cacher 实例来存放闭包
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);

    let x = 10;
    /* fn is_equal(z: u32) -> bool {
        z == x // can't capture dynamic environment in a fn item
    } */
    let is_equal = |z| z == x;
    println!("x = {}, z = {}, is_equal = {}", x, 10, is_equal(10));

    let v = vec![0, 1, 2];
    println!("v = {:?}", v);
    let equal_to_v = move |z| z == v;
    //println!("v = {:?}", v); // borrow of moved value: `v`
    let y = vec![0, 1, 2];
    assert!(equal_to_v(y));
}

#[cfg(test)]
mod tests {
    #[test]
    fn call_with_different_values() {
        let mut c = super::Cacher::new(|a| a);
        let v1 = c.value(1);
        let v2 = c.value(2);
        assert_eq!(v2, 2);
    }
}
