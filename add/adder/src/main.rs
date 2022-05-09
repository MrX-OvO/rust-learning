use add_one;
use add_rand;
use add_tow;

fn main() {
    let num = 10;
    let (result, rand) = add_rand::add_rand(num);
    println!(
        "num = {}, num + 1 = {}, num + 2 = {}, num + rand(={}) = {}",
        num,
        add_one::add_one(num),
        add_tow::add_tow(num),
        rand,
        result
    );
}
