struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push(&mut self, value: T) {
        self.head = Some(Box::new(Node {
            value,
            next: self.head.take(),
        }));
    }

    pub fn pop(&mut self) -> Option<T> {
        let mut old_head = self.head.take();
        self.head = old_head.as_mut().and_then(|node| node.next.take());
        old_head.map(|node| node.value)
    }
}

pub struct LinkedListMutIterator<'list, T> {
    node: Option<&'list mut Node<T>>,
}

impl<'list, T> Iterator for LinkedListMutIterator<'list, T> {
    type Item = &'list mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.node.take().map(|node| {
            self.node = node.next.as_deref_mut();
            &mut node.value
        })
    }
}

impl<T> LinkedList<T> {
    pub fn iter_mut(&mut self) -> LinkedListMutIterator<T> {
        LinkedListMutIterator {
            node: self.head.as_deref_mut(),
        }
    }
}

pub struct LinkedListIterator<'list, T> {
    node: Option<&'list Node<T>>,
}

impl<'list, T> Iterator for LinkedListIterator<'list, T> {
    type Item = &'list T;
    fn next(&mut self) -> Option<Self::Item> {
        self.node.map(|node| {
            self.node = node.next.as_deref();
            &node.value
        })
    }
}

impl<T> LinkedList<T> {
    pub fn iter(&self) -> LinkedListIterator<T> {
        LinkedListIterator {
            node: self.head.as_deref(),
        }
    }
}

pub struct LinkedListIntoIterator<T> {
    list: LinkedList<T>,
}

impl<T> Iterator for LinkedListIntoIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop()
    }
}

impl<T> LinkedList<T> {
    pub fn into_iter(self) -> LinkedListIntoIterator<T> {
        LinkedListIntoIterator { list: self }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut list = LinkedList::<i32>::new();
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.pop().unwrap(), 3);
        assert_eq!(list.pop().unwrap(), 2);
        assert_eq!(list.pop().unwrap(), 1);
        assert_eq!(list.pop(), None);
        list.push(4);
        list.push(5);
        list.push(6);
        let mut iter = list.iter();
        assert_eq!(*iter.next().unwrap(), 6);
        assert_eq!(*iter.next().unwrap(), 5);
        assert_eq!(*iter.next().unwrap(), 4);
        list.iter_mut().for_each(|val| *val += 1);
        assert_eq!(list.pop().unwrap(), 7);
        assert_eq!(list.pop().unwrap(), 6);
        assert_eq!(list.pop().unwrap(), 5);
        assert_eq!(list.pop(), None);
        assert_eq!(list.pop(), None);
    }
}
