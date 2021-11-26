fn main() {
    let s = String::from("Hello, world!");
    takeownership(s);
    //println!("after called func takeownership, s={}", s); // borrow of moved value: `s` value borrowed here after move

    let x = 1024;
    make_copy(x);
    println!("after called func make_copy, x={}", x);
}

fn takeownership(owners: String) {
    println!("Takeownership: {:?}", owners);
}

fn make_copy(somes: i32) {
    println!("somes: {:?}", somes);
}
