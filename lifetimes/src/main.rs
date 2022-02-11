fn main() {
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

    // 标注生命周期 'a
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}
