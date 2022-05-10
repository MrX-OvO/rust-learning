use std::rc::Rc;
#[derive(Debug)]
enum List {
    //Cons(i32, Box<List>),
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

struct CustomSmartPointer {
    data: String,
}

use std::ops::Drop;
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}` !", self.data);
    }
}

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    //let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    //println!("list = {:?}", list);

    let x = 5;
    let y = &x;
    assert_eq!(x, 5);
    assert_eq!(*y, 5);

    let y = Box::new(x);
    assert_eq!(*y, 5);

    let y = MyBox::new(x);
    assert_eq!(*y, 5);

    let s = MyBox::new(String::from("Rust"));
    hello(&s);

    let a = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    drop(a);
    let b = CustomSmartPointer {
        data: String::from("others"),
    };
    println!("CustomSmartPointers created!");

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
