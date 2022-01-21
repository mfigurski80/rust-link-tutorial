use std::mem;

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }
    pub fn push(&mut self, elem: i32) {
        let new_node = Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        };
        self.head = Link::More(Box::new(new_node));
    }
    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut l = List::new();
        // Check empty list behavior
        assert_eq!(l.pop(), None);
        // Populate
        l.push(1);
        l.push(2);
        l.push(3);
        // Check removal
        assert_eq!(l.pop(), Some(3));
        assert_eq!(l.pop(), Some(2));
        // Push more
        l.push(4);
        l.push(5);
        // Check removal
        assert_eq!(l.pop(), Some(5));
        assert_eq!(l.pop(), Some(4));
        // Check exhaustion
        assert_eq!(l.pop(), Some(1));
        assert_eq!(l.pop(), None);
    }
}
