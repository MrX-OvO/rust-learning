const GLOBAL_CONSTANT: u32 = 100;
fn main() {
    let x = 6;
    //x = x + 1; // error: cannot assign twice to immutable variable 'x'
    println!("immutable variable x:{}", x);

    let mut y = 6;
    println!("mutable variable y:{}", y);
    y = x + 1;
    println!("mutable variable y(=x({})+1):{}", x, y);
    y = y * 2;
    println!("shadow y:{}", y);

    const LOCAL_CONSTANT: u32 = 100;
    println!("local constant CONSTANT:{}", LOCAL_CONSTANT);
    println!("global constant CONSTANT:{}", GLOBAL_CONSTANT);
}
