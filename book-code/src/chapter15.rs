use std::{ops::Deref, rc::Rc};

pub fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    recursive_type_with_box();

    deref_coresion();

    drop_trait();

    reference_counted();

    ref_cell_interior_mutability::main();

    memory_leaks::main();

    memory_leaks::tree_main();
}

fn recursive_type_with_box() {
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    let list = List::Cons(8, Box::new(List::Cons(9, Box::new(List::Nil))));

    println!("{:#?}", list);
}

fn deref_coresion() {
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> Self {
            Self(x)
        }
    }

    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &T {
            &self.0
        }
    }

    impl<T> Drop for MyBox<T> {
        fn drop(&mut self) {
            println!("Dropping MyBox with data");
        }
    }

    let m = MyBox::new("Rusty".to_string());
    hello(&m);
}

fn hello(name: &str) {
    println!("Hello, {}", name);
}

fn drop_trait() {
    #[derive(Debug)]
    struct CustomSmartPointer {
        data: String,
    }

    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data `{}`!", self.data);
        }
    }

    let c = CustomSmartPointer {
        data: "Very Important Stuff".to_string(),
    };

    let d = CustomSmartPointer {
        data: "Another important thing".to_string(),
    };

    println!("{:#?}", c);
    println!("{:#?}", d)
}

fn reference_counted() {
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }

    fn print_count<T>(ref_count: &Rc<T>, message: &str) {
        println!("{} = {}", message, Rc::strong_count(ref_count));
    }

    let a = Rc::new(List::Cons(5, Rc::new(List::Cons(10, Rc::new(List::Nil)))));
    print_count(&a, "count after creating a");
    let _b = List::Cons(3, Rc::clone(&a));
    print_count(&a, "count after creating b");
    {
        let _c = List::Cons(4, Rc::clone(&a));
        print_count(&a, "count after creating c");
    }
    print_count(&a, "count afer c goes out of scope");
}

mod ref_cell_interior_mutability {
    use std::{cell::RefCell, rc::Rc};

    pub trait Messenger {
        fn send(&self, msg: &str);
    }

    pub struct LimitTracker<'a, T: Messenger> {
        messenger: &'a T,
        value: usize,
        max: usize,
    }

    impl<'a, T> LimitTracker<'a, T>
    where
        T: Messenger,
    {
        pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
            LimitTracker {
                messenger,
                value: 0,
                max,
            }
        }

        pub fn set_value(&mut self, value: usize) {
            self.value = value;

            let percentage_of_max = self.value as f64 / self.max as f64;

            if percentage_of_max >= 1.0 {
                self.messenger.send("Error: You are over your quota!");
            } else if percentage_of_max >= 0.9 {
                self.messenger
                    .send("Urgent warning: You've used up over 90% of your quota!");
            } else if percentage_of_max >= 0.75 {
                self.messenger
                    .send("Warning: You've used up over 75% of your quota!");
            }
        }
    }

    #[derive(Debug)]
    enum List {
        Cons(Rc<RefCell<i32>>, Rc<List>),
        Nil,
    }

    pub fn main() {
        let value = Rc::new(RefCell::new(5));
        let a = Rc::new(List::Cons(Rc::clone(&value), Rc::new(List::Nil)));
        let b = List::Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
        let c = List::Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

        *value.borrow_mut() += 10;

        println!("a after = {:?}", a);
        println!("b after = {:?}", b);
        println!("c after = {:?}", c);
    }

    #[cfg(test)]
    mod test {
        use std::cell::RefCell;

        use super::*;

        struct MockMessenger {
            sent_messages: RefCell<Vec<String>>,
        }

        impl MockMessenger {
            fn new() -> Self {
                Self {
                    sent_messages: RefCell::new(vec![]),
                }
            }
        }

        impl Messenger for MockMessenger {
            fn send(&self, message: &str) {
                self.sent_messages.borrow_mut().push(message.to_string())
                // let mut one_borrow = self.sent_messages.borrow_mut();
                // let second_borrow = self.sent_messages.borrow();

                // one_borrow.push(message.to_string());
                // second_borrow.push(message.to_string());
            }
        }

        #[test]
        fn it_sends_an_over_75_percent_warning_message() {
            let mock_messenger = MockMessenger::new();
            let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

            limit_tracker.set_value(80);

            assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
        }
    }
}

mod memory_leaks {
    use std::{
        borrow::Borrow,
        cell::RefCell,
        rc::{Rc, Weak},
    };

    use List::{Cons, Nil};

    type ConsList = RefCell<Rc<List>>;
    #[derive(Debug)]
    enum List {
        Cons(i32, ConsList),
        Nil,
    }

    impl List {
        fn tail(&self) -> Option<&ConsList> {
            match self {
                Cons(_, item) => Some(item),
                Nil => None,
            }
        }
    }

    pub fn main() {
        let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

        println!("a initial rc count = {}", Rc::strong_count(&a));
        println!("a next item = {:?}", a.tail());

        let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

        println!("a rc count after b creation = {}", Rc::strong_count(&a));
        println!("b initial rc count = {}", Rc::strong_count(&b));
        println!("b next item = {:?}", b.tail());

        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b);
        }

        println!("b rc count after changing a = {}", Rc::strong_count(&b));
        println!("a rc count after changing a = {}", Rc::strong_count(&a));

        // Uncomment the next line to see thatwe have a cycle;
        // it will overflow the stack
        //println!("a next item = {:?}", a.tail());
    }

    // creating a tree
    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>,
        children: RefCell<Vec<Rc<Node>>>,
    }

    pub fn tree_main() {
        let leaf = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });

        println!(
            "leaf strong = {}, leaf weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );

        {
            let branch = Rc::new(Node {
                value: 5,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![Rc::clone(&leaf)]),
            });

            *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

            println!(
                "branch strong = {}, weak = {}",
                Rc::strong_count(&branch),
                Rc::weak_count(&branch),
            );

            println!(
                "leaf strong = {}, weak = {}",
                Rc::strong_count(&leaf),
                Rc::weak_count(&leaf),
            );
        }

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }
}
