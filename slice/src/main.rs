fn main() {
    let s = String::from("Hello world!");
    let first_word = find_first_word(&s);
    println!("The first word of `{}` is {}", s, first_word);

    let arr = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let slice = &arr[1..5];
    for (i, &item) in slice.iter().enumerate() {
        println!("slice[{}]: {}", i, item);
    }
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
