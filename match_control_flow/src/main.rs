#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let penny = Coin::Penny;
    let penny_value = value_in_cents(&penny);
    println!("Penny is {:?}, value is {}", penny, penny_value);
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Penny");
            1
        }
        Coin::Nickel => {
            println!("Nickel");
            5
        }
        Coin::Dime => {
            println!("Dime");
            10
        }
        Coin::Quarter => {
            println!("Quarter");
            25
        }
    }
}
