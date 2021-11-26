fn main() {
    // stack Types
    let x = 5;
    let y = x;
    println!("x:{} y:{}", x, y);
    // heap Types
    let s1 = String::from("Hello world!");
    //let s2 = s1;
    //println!("s1:{}, s2:{}", s1, s2); // borrow of moved value: `s1` value borrowed here after move
    let s2 = s1.clone(); // clone 主要针对heap上的数据，且比较消耗资源
    println!("s1:{}, s2:{}", s1, s2);
}
