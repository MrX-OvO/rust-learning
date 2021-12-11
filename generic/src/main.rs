// 1.函数泛型
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// 2.struct泛型
struct Point<T, U> {
    x: T,
    y: U,
}

// 3.enum泛型，让枚举的变体持有泛型数据类型
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

// 3.方法泛型
impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &U {
        &self.y
    }
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let list = vec![1, 2, 3, 4, 5, 6];
    let largest = largest(&list);
    println!("largest in list({:?}) is {}", list, largest);

    let integer_p = Point { x: 10, y: 20 };
    let p = Point { x: 10.2, y: 20 };
    println!("p.x() is {}", p.x());

    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: "Hello", y: 'w' };
    let p3 = p1.mixup(p2);
    println!("p3.x() is {}, p3.y() is {}", p3.x(), p3.y());
}
