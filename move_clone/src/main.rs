fn main() {
    let s1 = String::from("Hello world!");
    //let s2 = s1;
    //println!("s1:{}, s2:{}", s1, s2); // borrow of moved value: `s1` value borrowed here after move
    let s2 = s1.clone();
    println!("s1:{}, s2:{}", s1, s2);
}
