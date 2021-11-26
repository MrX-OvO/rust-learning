fn main() {
    let s = String::from("Hello, world!");
    takeownership(s);
    //println!("after called func takeownership, s={}", s); // borrow of moved value: `s` value borrowed here after move

    let x = 1024;
    make_copy(x);
    println!("after called func make_copy, x={}", x);

    let required_str = givenowners();
    println!(
        "after called func givenowners, requiredStr={}",
        required_str
    );

    let s = take_and_give_back_ownership(required_str);
    println!("after called func take_and_give_back_ownership, s={}", s);
}

fn takeownership(owners: String) {
    println!("Takeownership: {:?}", owners);
}

fn make_copy(somes: i32) {
    println!("somes: {:?}", somes);
}

fn givenowners() -> String {
    let s = String::from("Hello");
    s
}

fn take_and_give_back_ownership(owners: String) -> String {
    owners
}
