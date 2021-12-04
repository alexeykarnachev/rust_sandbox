use std::rc::Rc;

pub struct PersistentLinkedList<T> {
    head: Option<Rc<Node<T>>>,
}

struct Node<T> {
    value: T,
    next: Option<Rc<Node<T>>>,
}

impl<T> PersistentLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push(&mut self, value: T) {
        self.head = Some(Rc::new(Node {
            value,
            next: self.head.take(),
        }));
    }

    pub fn tail(&self) -> Option<Self> {
        self.head.as_ref().map(|node| Self {
            head: node.next.clone(),
        })
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    pub fn prepend(&self, value: T) -> Self {
        Self {
            head: Some(Rc::new(Node {
                value,
                next: self.head.clone(),
            })),
        }
    }

    pub fn iter(&self) -> PersistentLinkedListIterator<T> {
        PersistentLinkedListIterator {
            node: self.head.as_deref(),
        }
    }
}

pub struct PersistentLinkedListIterator<'a, T> {
    node: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for PersistentLinkedListIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.node.map(|node| {
            self.node = node.next.as_deref();
            &node.value
        })
    }
}

#[cfg(test)]
mod tests {
    use super::PersistentLinkedList;

    #[test]
    fn it_works() {
        let mut a = PersistentLinkedList::<i32>::new();
        a.push(1); // a --> 1
        a.push(2); // a --> 2 --> 1
        a.push(3); // a --> 3 --> 2 --> 1
        let mut a_it = a.iter();
        assert_eq!(a_it.next(), Some(&3));
        assert_eq!(a_it.next(), Some(&2));
        assert_eq!(a_it.next(), Some(&1));
        assert_eq!(a_it.next(), None);

        let mut b = a.tail().unwrap(); // b --> 2 --> 1
        let mut b_it = b.iter();
        assert_eq!(b_it.next(), Some(&2));
        assert_eq!(b_it.next(), Some(&1));
        assert_eq!(b_it.next(), None);

        b.push(5); // b --> 5 --> 2 --> 1
        b.push(6); // b --> 6 --> 5 --> 2 --> 1
        let mut b_it = b.iter();
        assert_eq!(b_it.next(), Some(&6));
        assert_eq!(b_it.next(), Some(&5));
        assert_eq!(b_it.next(), Some(&2));
        assert_eq!(b_it.next(), Some(&1));
        assert_eq!(b_it.next(), None);

        let c = b.prepend(8); // c --> 8 --> 6 --> 5 --> 2 --> 1
        let mut c_it = c.iter();
        assert_eq!(c_it.next(), Some(&8));
        assert_eq!(c_it.next(), Some(&6));
        assert_eq!(c_it.next(), Some(&5));
        assert_eq!(c_it.next(), Some(&2));
        assert_eq!(c_it.next(), Some(&1));
        assert_eq!(c_it.next(), None);

        // b --> 6 --> 5 --> 2 --> 1
        let mut b_it = b.iter();
        assert_eq!(b_it.next(), Some(&6));
        assert_eq!(b_it.next(), Some(&5));
        assert_eq!(b_it.next(), Some(&2));
        assert_eq!(b_it.next(), Some(&1));
        assert_eq!(b_it.next(), None);

        // a --> 3..., b --> 6..., c --> 8...
        assert_eq!(a.head(), Some(&3));
        assert_eq!(b.head(), Some(&6));
        assert_eq!(c.head(), Some(&8));

        // d --> None
        let d = PersistentLinkedList::<i32>::new();
        assert_eq!(d.head(), None);
        assert_eq!(d.tail().is_none(), true);
        let mut d_it = d.iter();
        assert_eq!(d_it.next(), None);
    }
}
