fn main() {
    let s = String::from("Hello world!");
    let first_word = find_first_word(&s);
    println!("The first word of `{}` is {}", s, first_word);
}

// 查找字符串第一个单词
fn find_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
