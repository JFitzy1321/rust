pub enum BTNode<T> {
    Leaf(T),
    Branch {
        left: Box<BTNode<T>>,
        right: Box<BTNode<T>>,
        op: Box<dyn Fn(T, T) -> T>,
    },
}

impl<T> BTNode<T> where T: std::ops::Add<Output = T> {
    pub fn add<L, R>(left: L, right: R) -> Self
    where
    L:Into<BTNode<T>>,
    R: Into<BTNode<T>>, {
        BTNode::Branch{
            left: Box::new(left.into()),
            right: Box::new(right.into()),
            op: Box::new(|l, r| l + r)
        }
    }
}

impl<T> From<T> for BTNode<T> {
    fn from(t: T) -> BTNode<T> {
        BTNode::Leaf(t)
    }
}

impl<T> BTNode<T> {
    pub fn new(t: T) -> Self {
        BTNode::Leaf(t)
    }

    pub fn value(self) -> T {
        match self {
            Self::Leaf(t) => t,
            Self::Branch{left, right, op} => op(left.value(), right.value()),
        }
    }
}

struct BinaryTree<T> {
    head: Option<BTNode<T>>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let tree = BTNode::add(10, )
    }
}
