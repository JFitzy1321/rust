use core::ptr::NonNull;

pub struct LinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    size: usize,
}

impl<T> LinkedList<Node<T>> {
    pub fn add(&mut self, item: T) {}
}

pub struct Node<T> {
    item: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(item: T) -> Self {
        Node {
            next: None,
            prev: None,
            item: item,
        }
    }
    pub fn next(&self) -> Option<Self> {
        self.Next
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
