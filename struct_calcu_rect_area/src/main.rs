#[derive(Debug)]
struct Rectangle {
    width: usize,
    height: usize,
}

fn main() {
    let w = 30;
    let l = 50;
    println!("calling func area1(), result:{}", area1(w, l));

    let tuple_of_rect = (30, 50);
    println!("calling func area2(), result:{}", area2(tuple_of_rect));

    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("calling func area3(), result:{}", area3(&rect));
    println!("{:#?}", rect);
}

fn area1(width: usize, height: usize) -> usize {
    width * height
}

fn area2(dim: (usize, usize)) -> usize {
    dim.0 * dim.1
}

fn area3(rect: &Rectangle) -> usize {
    rect.width * rect.height
}
