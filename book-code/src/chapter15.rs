use std::{ops::Deref, rc::Rc};

pub fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    recursive_type_with_box();

    deref_coresion();

    drop_trait();

    reference_counted();

    ref_cell_interior_mutability();
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

fn ref_cell_interior_mutability() {}
