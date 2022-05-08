fn main() {
    let v = vec![1, 2, 3, 4];
    let v_iter = v.iter();

    for item in v_iter {
        println!("Got: {}", item);
    }

    //println!("v = {:?}", v_iter); // borrow of moved value: `v_iter`
}
