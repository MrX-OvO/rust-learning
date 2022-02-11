// 3.结构体定义中生命周期标注（引用）
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    //1.
    /*  let r;
    {
        let x = 5;
        r = &x; // `x` does not live long enough borrowed value does not live long enough
    }
    println!("r: {}", r); */
    let string1 = String::from("rust");
    let string2 = "hello";

    let result = longest(string1.as_str(), string2);

    println!("The longest string is {}", result);

    // 2.标注生命周期 'a
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");

    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
