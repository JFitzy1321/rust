pub fn main() {
    unsafe_rust::main();
    advanced_traits::main();
}

mod unsafe_rust {
    // Unsafe code gives access to these five "unsafe superpowers"
    // - Dereference raw pointers
    //      -- *const T (immutable pointer)
    //      -- *mut T (mutable pointer)
    // - Call unsafe functions or methods
    // - Implement unsafe traits
    // - Access fields of `union`s (wtf is a union in rust)
    // It's also need for FFI (Foreign Function Interface) and accessing hardware
    // Because these things are 'outside' the scope of the compiler and borrow checker,
    // rust cannot guarantee saftey with this operations
    pub fn main() {
        let mut num = 5;

        // we can create raw pointers in 'safe' code
        // but we can't dereference them here
        let r1 = &num as *const i32;
        let r2 = &mut num as *mut i32;
        unsafe {
            println!("r1 is: {}", *r1);
            println!("r2 is: {}", *r2);

            dangerous();

            println!("Absolute value of -3, according to C Library: {}", abs(-3));
        }

        // raw pointer to arbitrary memory
        // let address = 0x012345usize;
        // let r = address as *const i32;
    }

    unsafe fn dangerous() {}

    use std::slice;

    fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = slice.len();
        let ptr = slice.as_mut_ptr();

        assert!(mid <= len);

        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }

    // Call a C ABI
    extern "C" {
        fn abs(input: i32) -> i32;
    }

    // write a funcion to be used by c
    // pub extern "C" fn call_from_c() {
    //     println!("Just called a Rust function from C!");
    // }

    unsafe trait Foo {}

    unsafe impl Foo for i32 {}
}

mod advanced_traits {
    pub trait MyIterator {
        // Item is an Associated Type.
        // Anyone implementing this trait must provide
        // a concrete implementation for Item
        // i.e. `type Item = i32`
        type Item;

        fn next(&mut self) -> Option<Self::Item>;
    }

    // Disambiguation
    // what happens if your type, or multiple traits, have the same signature?

    trait Pilot {
        fn fly(&self);
    }

    trait Wizard {
        fn fly(&self);
    }

    struct Human;

    impl Pilot for Human {
        fn fly(&self) {
            println!("This is your captain speaking.");
        }
    }

    impl Wizard for Human {
        fn fly(&self) {
            println!("**Harry Potter Noises start**");
        }
    }

    impl Human {
        fn fly(&self) {
            println!("You can't fly, puny hooman");
        }
    }

    trait Animal {
        fn baby_name() -> String;
    }

    struct Dog;

    impl Dog {
        fn baby_name() -> String {
            "Spot".to_string()
        }
    }

    impl Animal for Dog {
        fn baby_name() -> String {
            "puppy".to_string()
        }
    }

    // SUPER TRAITS (it's basically inheritance for traits, but don't say that in a rust chat)
    trait Print: std::fmt::Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }

    struct Point;

    impl std::fmt::Display for Point {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            todo!()
        }
    }

    impl Print for Point {}

    // NewType Pattern, a Wrapper around other types.
    // Wrapper must be a Tuple Struct of desiered type.
    // Then implement traits on that Wrapper
    struct MyWrapper(Vec<String>);

    trait Something {}

    impl Something for MyWrapper {}

    pub fn main() {
        let dude = Human;

        Pilot::fly(&dude);
        Wizard::fly(&dude);
        // this will default to the Human impl block
        dude.fly();

        // calling assoicated functions

        // this one will call Dog::baby_name
        println!("A baby does is called a {}", Dog::baby_name());

        // Fully Qualified Syntax is needed to call the associated trait function
        println!("A baby does is called a {}", <Dog as Animal>::baby_name());
    }
}

mod advanced_types {
    // / ! means 'never' type, as in never returns.
    // This is used for things like the continue keyword, loop, and panic! .
    // These things do not return anything, but is this necessary?
    // Mainly, the type system and pattern matching.
    // In a match statement, if we assing the results to a variable, all arms of the match
    // statement must return the same type of thing. But, what if panic is used in the Error
    // arm? '!' Will downcast to whatever other type is used, because it doesn't return anything
    // it can be cast to anything.

    // Sized and ?Sized traits are respectively: For things that have a known size at compile time
    // and things that have a known or unknown size at compile time.
    // Generics automatically assumed the Sized trait, so to use dynamic sized types '<T: ?Sized>(t: &T)' is needed.
    // Also, anthing that is unknown size (?Sized) must be behind a pointer (or reference).
}

mod advanced_functions_and_closures {
    fn add_one(x: i32) -> i32 {
        x + 1
    }

    // lowercase fn() is a function pointer
    // uppercase Fn is the Closure Trait
    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }

    // returning a closure
    fn return_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }

    fn main() {
        let answer = do_twice(add_one, 5);

        println!("The answer is {}", answer);

        let list_of_numbers = vec![1, 2, 3];
        // let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
        let list_of_string: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
    }
}

mod macros {
    // wtf is dat m8ty?
    // Do more reseach when needed.
}
