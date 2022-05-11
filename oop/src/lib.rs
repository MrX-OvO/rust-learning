pub trait Draw {
    fn draw(&self) {
        println!("draw...");
    }
}

// Box<dyn<Trait>> Trait对象，集合可以包含不同的元素，如Button、TextFiled
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

/* 适合同质集合，例如都是Button
pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}*/

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
    // ...
}

impl Draw for Button {
    fn draw(&self) {
        println!("draw button...");
    }
}
