use std::{fs::File, io::ErrorKind};

fn main() {
    let f = File::open("./hello.txt").unwrap();

    let f = File::open("./hello.txt").expect("无法打开文件 hello.txt");

    let f = File::open("./hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("./hello.txt") {
                Ok(file) => file,
                Err(error) => {
                    panic!("Error creating file: {:?}", error);
                }
            },
            other_error => panic!("Error opening file: {:?}", other_error),
        },
    };

    println!("{:?}", f);
}
