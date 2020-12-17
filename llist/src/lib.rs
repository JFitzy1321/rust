// // naive attempt at LinkedList
// enum Node<T> {
//     Empty,
//     NonEmpty(T, Box<Node<T>>),
// }

// Better, but still not great, attempt
type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

pub struct LinkedList<T> {
    head: Link<T>,
    len: usize,
}

impl<T> LinkedList<T> {
    pub fn empty() -> Self {
        Self { len: 0, head: None }
    }

    pub fn push(&mut self, elem: T) {
        self.head = Some(Box::new(Node {
            elem,
            next: self.head.take(),
        }));
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|n| {
            self.len -= 1;
            self.head = n.next;
            n.elem
        })
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|n| &n.elem)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty_works() {
        let list: LinkedList<u32> = LinkedList::empty();
        assert!(list.head.is_none());
        assert_eq!(list.len, 0);
    }

    mod push {
        use super::*;
        #[test]
        fn simple_works() {
            let mut list = LinkedList::empty();
            list.push(42);
            assert_eq!(list.head.unwrap().elem, 42);
        }
    }

    mod len {
        use super::*;
        #[test]
        fn works_after_empty() {
            let list: LinkedList<u32> = LinkedList::empty();
            assert_eq!(list.len(), 0)
        }

        #[test]
        fn works_after_push() {
            let mut list = LinkedList::empty();
            list.push(42);
            list.push(69);
            list.push(144);
            assert_eq!(list.len(), 3);
        }
    }

    mod pop {
        use super::*;

        #[test]
        fn none_when_head_is_none() {
            let mut list: LinkedList<u32> = LinkedList::empty();
            assert_eq!(list.pop(), None);
        }

        #[test]
        fn some_then_none_after_push() {
            let mut list = LinkedList::empty();
            list.push(42);
            assert_eq!(list.pop(), Some(42));
            assert_eq!(list.pop(), None);
        }
    }

    mod peek {
        use super::*;

        #[test]
        fn returns_none_when_empty() {
            let list: LinkedList<u32> = LinkedList::empty();
            assert_eq!(list.peek(), None);
        }

        #[test]
        fn returns_some_ref() {
            let mut list = LinkedList::empty();
            list.push(42);
            assert_eq!(list.peek(), Some(&42));
        }
    }
}
