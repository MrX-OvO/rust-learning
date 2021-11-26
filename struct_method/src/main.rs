#[derive(Debug)]
struct Rectangle {
    width: usize,
    height: usize,
}

// impl 实现 struct Rectangle 接口，可以有多个impl块
impl Rectangle {
    // method with self or &self or &mut self
    fn area(&self) -> usize {
        self.width * self.height
    }

    // method with parameters include self
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    // function, for constructor, use operator `::` to call this function
    fn square(size: usize) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "calling struct `Rectangle` area() method, result:{}",
        rect.area()
    );
    println!("{:#?}", rect);

    let rect1 = Rectangle {
        width: 20,
        height: 10,
    };
    let rect2 = Rectangle {
        width: 20,
        height: 60,
    };
    println!("rect can hold rect1? {}", rect.can_hold(&rect1));
    println!("rect can hold rect2? {}", rect.can_hold(&rect2));

    let square_size = 10;
    let square = Rectangle::square(square_size);
    println!(
        "calling struct `Rectangle` area() method, result:{}",
        square.area()
    );
    println!("{:#?}", square);
    println!("rect can hold square? {}", rect.can_hold(&square));
}
