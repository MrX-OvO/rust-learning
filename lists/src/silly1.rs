pub struct Stack<T> {
    head: Link<T>,
}

struct Node<T> {
    element: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { head: None }
    }

    pub fn push(&mut self, element: T) {
        let new_node = Box::new(Node {
            element: element,
            next: None,
        });

        self.push_node(new_node);
    }

    fn push_node(&mut self, mut node: Box<Node<T>>) {
        node.next = self.head.take();
        self.head = Some(node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.pop_node().map(|node| node.element)
    }

    fn pop_node(&mut self) -> Option<Box<Node<T>>> {
        self.head.take().map(|mut node| {
            self.head = node.next.take();
            node
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.element)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.element)
    }
}

impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

struct List<T> {
    left: Stack<T>,
    right: Stack<T>,
}

impl<T> List<T> {
    fn new() -> Self {
        List {
            left: Stack::new(),
            right: Stack::new(),
        }
    }

    pub fn push_left(&mut self, element: T) {
        self.left.push(element)
    }

    pub fn push_right(&mut self, element: T) {
        self.right.push(element)
    }

    pub fn pop_left(&mut self) -> Option<T> {
        self.left.pop()
    }
    pub fn pop_right(&mut self) -> Option<T> {
        self.right.pop()
    }

    pub fn peek_left(&self) -> Option<&T> {
        self.left.peek()
    }

    pub fn peek_right(&self) -> Option<&T> {
        self.right.peek()
    }

    pub fn peek_left_mut(&mut self) -> Option<&mut T> {
        self.left.peek_mut()
    }

    pub fn peek_right_mut(&mut self) -> Option<&mut T> {
        self.right.peek_mut()
    }

    pub fn go_left(&mut self) -> bool {
        self.left
            .pop_node()
            .map(|node| {
                self.right.push_node(node);
            })
            .is_some()
    }

    pub fn go_right(&mut self) -> bool {
        self.right
            .pop_node()
            .map(|node| {
                self.left.push_node(node);
            })
            .is_some()
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn walk_aboot() {
        let mut list = List::new(); // [_]

        list.push_left(0); // [0,_]
        list.push_right(1); // [0, _, 1]
        assert_eq!(list.peek_left(), Some(&0));
        assert_eq!(list.peek_right(), Some(&1));

        list.push_left(2); // [0, 2, _, 1]
        list.push_left(3); // [0, 2, 3, _, 1]
        list.push_right(4); // [0, 2, 3, _, 4, 1]

        while list.go_left() {} // [_, 0, 2, 3, 4, 1]

        assert_eq!(list.pop_left(), None);
        assert_eq!(list.pop_right(), Some(0)); // [_, 2, 3, 4, 1]
        assert_eq!(list.pop_right(), Some(2)); // [_, 3, 4, 1]

        list.push_left(5); // [5, _, 3, 4, 1]
        assert_eq!(list.pop_right(), Some(3)); // [5, _, 4, 1]
        assert_eq!(list.pop_left(), Some(5)); // [_, 4, 1]
        assert_eq!(list.pop_right(), Some(4)); // [_, 1]
        assert_eq!(list.pop_right(), Some(1)); // [_]

        assert_eq!(list.pop_right(), None);
        assert_eq!(list.pop_left(), None);
    }
}
