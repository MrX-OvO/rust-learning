fn main() {
    let r;
    {
        let x = 5;
        r = &x; // `x` does not live long enough borrowed value does not live long enough
    }
    println!("r: {}", r);
}
