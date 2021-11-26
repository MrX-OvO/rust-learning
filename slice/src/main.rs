fn main() {
    let mut s = String::from("Hello, world!");
    let word_index = find_first_word(&s);
    s.clear();
    println!(
        "The first-word's index range of `{}` is 0-{}",
        s, word_index
    );
}

// 查找字符串第一个单词
fn find_first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
