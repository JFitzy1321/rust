pub struct LinkedList<T> {
    head: Option<Node<T>>,
    tail: Option<Node<T>>,
    len: usize,
}

impl<T> LinkedList<Node<T>> {
    pub fn add(&mut self, item: T) {}
}

pub struct Node<T> {
    item: T,
    next: Option<Box<Node<T>>>,
    prev: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(item: T) -> Self {
        Node {
            next: None,
            prev: None,
            item: item,
        }
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn new_with(item: T) -> Self {
        LinkedList {
            head: Some(Box::new(Node::new(item))),
            tail: Some(Box::new(elem)),
            len: 1,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
