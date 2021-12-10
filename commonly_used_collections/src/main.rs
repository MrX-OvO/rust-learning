fn main() {
    // 1.Vector
    // let v: Vec<i32> = Vec::new(); // 创建空Vector，需要指定数据类型
    let v = vec![1, 2, 3, 4]; // vec!宏，用于创建Vector
    for item in v.iter() {
        println!("{}", item);
    }

    let mut v = Vec::new();
    for i in 0..5 {
        v.push(i);
    }
    for i in &mut v {
        *i += 50;
    }
    for i in &v {
        println!("{}", i);
    }

    let third = v[2];
    println!("The thrid element of vector v is {}", third);

    match v.get(2) {
        Some(third) => println!("The thrid element of vector v is {}", third),
        None => println!("There is no thrid element of vector v"),
    }
    /*
    let first = &v[0];
    v.push(10); // cannot borrow `v` as mutable because it is also borrowed as immutable
    println!("The first element of vector v is {}", first);*/

    // vector with enum
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(3.14),
        SpreadsheetCell::Text(String::from("blue")),
    ];
}
