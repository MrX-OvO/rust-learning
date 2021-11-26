fn main() {
    let mut s = String::from("Hello world!");
    let mut length = calculate_length(&s);
    println!("The length of the string `{}` is {}", s, length);

    length = calculate_mut_length(&mut s);
    println!(
        "After called calculate_mut_length, the length of the string `{}` is {}",
        s, length
    );

    /*let _s1 = &mut s;
    let _s2 = &mut s; // cannot borrow `s` as mutable more than once at a time
    println!("_s1:{}, _s2: {}", _s1, _s2);*/
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn calculate_mut_length(s: &mut String) -> usize {
    s.push_str(" Hello, rust!");
    s.len()
}
