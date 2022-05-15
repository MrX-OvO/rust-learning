struct Node<T> {
    element: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

pub struct List<T> {
    head: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, element: T) {
        let new_node = Box::new(Node {
            element,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.element
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.element)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.element)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();

        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        // access fields of a tuple struct numerically
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

// 这里无需生命周期，因为 List 没有使用生命周期的关联项
impl<T> List<T> {
    // 这里我们为 `iter` 声明一个生命周期 'a , 此时 `&self` 需要至少和 `Iter` 活得一样久
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }

    /* 应用生命周期消除原则
    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref(),
        }
    } */

    /* "显式生命周期消除"语法
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    } */
}

// 这里声明生命周期是因为下面的关联类型 Item 需要
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    // 这里无需更改，因为上面已经处理了.
    // Self 依然是这么棒
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.element
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

// 这里无需生命周期，因为 List 没有使用生命周期的关联项
impl<T> List<T> {
    // 这里我们为 `iter` 声明一个生命周期 'a , 此时 `&self` 需要至少和 `Iter` 活得一样久
    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }

    /* 应用生命周期消除原则
    pub fn iter_mut(&mut self) -> Iter<T> {
        Iter {
            next: self.head.as_deref(),
        }
    } */

    /* "显式生命周期消除"语法
    pub fn iter_mut(&mut self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    } */
}

// 这里声明生命周期是因为下面的关联类型 Item 需要
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    // 这里无需更改，因为上面已经处理了.
    // Self 依然是这么棒
    fn next<'b>(&'b mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.element
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));
        list.peek_mut().map(|value| *value = 42);

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }
}
