use std::fmt::Show;

pub enum BinaryTree {
    Leaf(T),
    Branch(T, Box<BinaryTree<T>>, Box<BinaryTree<T>>),
    Null,
}

impl<T> BinaryTree<T> {
    fn create_tree(list: Vec<T>) -> Self {
        fn insert_node<T: Copy + Ord + Show>(val: T, btree: Self) -> Self {
            todo!();
        }

        let mut tree = Null;
    }
}

//https://stackoverflow.com/questions/24394211/difficulty-with-rust-binary-tree-implementation
