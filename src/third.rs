use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }
    pub fn prepend(&self, elem: T) -> List<T> {
        List {
            head: Some(Rc::new(Node {
                elem: elem,
                next: self.head.clone(),
            })),
        }
    }
    pub fn tail(&self) -> List<T> {
        List {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }
    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn base() {
        let l: List<u8> = List::new();
        assert_eq!(l.head(), None);
        let l = l.prepend(1).prepend(2).prepend(3);
        assert_eq!(l.head(), Some(&3));

        let l = l.tail();
        assert_eq!(l.head(), Some(&2));
        let l = l.tail();
        assert_eq!(l.head(), Some(&1));
        let l = l.tail();
        assert_eq!(l.head(), None);

        let l = l.tail();
        assert_eq!(l.head(), None);
    }
}
