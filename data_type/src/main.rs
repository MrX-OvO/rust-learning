fn main() {
    // Scalar Types
    let i: i64 = -1;
    let u: u64 = 1;
    let f: f64 = 3.14;
    let b: bool = true;
    let c: char = 'C';
    println!(
        "Scalar Types \n i(type:i64):{}\n u(type:u64):{}\n f(type:f64):{}\n c(type:char):{}\n b(type:bool):{}\n",
        i, u,f, b, c
    );

    // Compound Types
    print!("Compound Types\n");
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    print!("tup:{},{},{}\n", tup.0, tup.1, tup.2);
    let (x, y, z) = tup;
    print!("tup:x:{},y:{},z:{}\n", x, y, z);

    let arr = [0, 1, 2, 3];
    for i in 0..4 {
        print!("arr[{}]={}\n", i, arr[i]);
    }
}
