fn main() {
    // 1.Vector
    println!("1.Vector");
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

    // 2.String
    println!("\n2.String");
    let mut s = String::new();
    let data = "initial content";
    s = data.to_string();
    println!("s=\"{}\"", s);
    s = String::from("initial content");
    println!("s=\"{}\"", s);
    let s1 = "initial content".to_string();
    println!("s1=\"{}\"", s1);

    // String UTF8 编码
    let hello = String::from("你好");
    println!("Chinese {}", hello);
    let hello = String::from("こんにちは");
    println!("Japanese {}", hello);
    let hello = String::from("hello");
    println!("English {}", hello);
    let hello = String::from("안녕하세요.");
    println!("Korean {}", hello);
    let hello = String::from("Olá");
    println!("Portuguese {}", hello);

    let mut s = String::from("hello");
    println!("s=\"{}\"", s);
    s.push_str(" world"); // let s1 = String::from(" world"); s.push_str(&s1);
    println!("s=\"{}\"", s);
    s.push(',');
    println!("s=\"{}\"", s);

    // 拼接字符串
    let s1 = String::from("Hello");
    let s2 = String::from(" World!");
    let s3 = s1 + &s2; // `+`类似fn add(self, s: &str) -> String {...}
    println!("s3=\"{}\"", s3);
    // println!("s1=\"{}\"", s1); // borrow of moved value: `s1` value borrowed here after move
    println!("s2=\"{}\"", s2);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("s=\"{}\"", s);
    let s1 = String::from("tic");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s=\"{}\"", s);

    // let s0 = s[0]; // the type `std::string::String` cannot be indexed by `{integer}`
    let mut s = String::from("Hello");
    let mut len = s.len();
    println!("s = \"{}\", len = {}", s, len);
    s = String::from("你好");
    len = s.len();
    println!("s = \"{}\", len = {}", s, len);
    println!("s.bytes()"); // bytes
    for b in s.bytes() {
        println!("{}", b);
    }
    println!("s.chars()"); // scalar values
    for b in s.chars() {
        println!("byte:{}, len:{}", b, b.len_utf8());
    }

    // 切割String
    let s = String::from("Здравствуйте");
    for c in s.chars() {
        println!("byte:{}, len:{}", c, c.len_utf8());
    }
    let s0 = &s[0..4];
    println!("s0 = \"{}\"", s0);

    // 3.HashMap
    println!("\n3.HashMap");
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Tom"), 80);
    scores.insert(String::from("Jerry"), 90);

    for (k, v) in &scores {
        println!("key:{}, value:{}", k, v);
    }
    let key = String::from("Tom");
    let score = scores.get(&key);
    match score {
        Some(s) => println!("score:{}", s),
        None => println!("key not exist"),
    }

    let names = vec![String::from("Tom"), String::from("Jerry")];
    let scores2 = vec![80, 90];
    let scores3: HashMap<_, _> = names.iter().zip(scores2.iter()).collect();

    for (k, v) in &scores3 {
        println!("key:{}, value:{}", k, v);
    }
    let key = String::from("Tom");
    let score = scores3.get(&key);
    match score {
        Some(s) => println!("score:{}", s),
        None => println!("key not exist"),
    }

    let field_key = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(&field_key, &field_value);
    println!("field_key:{}, file_value:{}", field_key, field_value);
    let mut map = HashMap::new();
    map.insert(field_key, field_value);
    // println!("field_key:{}, file_value:{}",field_key,field_value); // borrow of moved value: `field_key` `field_value`

    let mut map = HashMap::new();
    map.insert(String::from("Blue"), 50);
    map.entry(String::from("Yellow")).or_insert(90);
    println!("{:#?}", map);
    let e = map.entry(String::from("Blue"));
    println!("{:#?}", e);
    e.or_insert(70);

    let text = String::from("Hello world and hello my friend and j");
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // word不存在，返回0；word存在，返回对应的value；返回的都是可变引用
        *count += 1;
    }
    println!("{:#?}", map);
}
