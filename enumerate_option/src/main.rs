fn main() {
    let some_number = Some(1);
    let some_string = Some("Hello");
    let absent_number: Option<i32> = None;
    println!(
        "some_number: {:?}\nsome_string: {:?}\nabsent_number: {:?}",
        some_number, some_string, absent_number
    );

    let x: i8 = 5;
    let y: Option<i8> = Some(6);
    //let z = x + y; // cannot add `std::option::Option<i8>` to `i8` no implementation for `i8 + std::option::Option<i8>`
}
