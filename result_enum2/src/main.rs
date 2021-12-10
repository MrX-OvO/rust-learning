use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file_1() -> Result<String, io::Error> {
    let f = File::open("./usernames.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut f = File::open("./usernames.txt")?;
    /*     let mut f = match File::open("./usernames.txt") {
        Ok(file) => file,
        Err(error) => return Err(error),
    } */

    let mut s = String::new();
    f.read_to_string(&mut s)?;
    /* match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    } */
    Ok(s)
}

fn read_username_from_file_3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("./usernames.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    let mut result = read_username_from_file_1();
    println!("result from fn read_username_from_file_1()\n{:#?}", result);
    result = read_username_from_file_2();
    println!(
        "\nresult from fn read_username_from_file_2()\n{:#?}",
        result
    );
    result = read_username_from_file_3();
    println!(
        "\nresult from fn read_username_from_file_3()\n{:#?}",
        result
    );
}
