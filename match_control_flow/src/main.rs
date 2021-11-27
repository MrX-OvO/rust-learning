#[derive(Debug)]
enum USAState {
    Alabama,
    Alaska,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(USAState),
}

fn main() {
    let penny = Coin::Penny;
    let penny_value = value_in_cents(&penny);
    println!("Penny is {:?}, value is {}", penny, penny_value);

    let quarter_from_alabama = Coin::Quarter(USAState::Alabama);
    println!("quarter_from_alabama:{:?}", quarter_from_alabama);
    println!("{:?}", value_in_cents(&quarter_from_alabama));

    let x: Option<i32> = Some(1);
    let y = plus_one(&x);
    println!("x:{:?}, y:{:?}", x, y);

    // match要穷举所有匹配模式，可用 `_` 代替其他剩余模式
    let v = 1u8;
    match v {
        1 => println!("one"),
        3 => println!("three"),
        _ => println!("..."), // u8共有256种模式，`_` 代替剩余的模式
    }
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
        Coin::Quarter(state) => {
            println!("Quarter from {:?}", state);
            25
        }
    }
}

fn plus_one(x: &Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
