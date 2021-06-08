// OOP in Rust
pub fn main() {
    trait_objects::main();
}
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        match self.list.pop() {
            Some(val) => {
                self.update_average();
                Some(val)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

mod trait_objects {
    pub trait Draw {
        fn draw(&self);
    }

    // Using 'Box<dyn trait-name>' instead of a trait bound
    // allows for multiple types to used
    pub struct Screen {
        // components could hold buttons, ListViews, Scrollbars, text, etc.
        pub components: Vec<Box<dyn Draw>>,
    }

    // This apporach only allows one type to be used at runtime
    // So Screen would only have a list of buttons or text
    // pub struct Screen<T: Draw> {
    //     pub components: Vec<T>,
    // }

    impl Screen {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }

    pub struct Button {
        pub width: u32,
        pub height: u32,
        pub label: String,
    }

    impl Draw for Button {
        fn draw(&self) {
            //code to draw a button
        }
    }

    pub struct SelectBox {
        width: u32,
        height: u32,
        options: Vec<String>,
    }

    impl Draw for SelectBox {
        fn draw(&self) {}
    }

    pub fn main() {
        let screen = Screen {
            components: vec![
                Box::new(SelectBox {
                    width: 75,
                    height: 10,
                    options: vec!["Yes".to_string(), "Maybe".to_string(), "No".to_string()],
                }),
                Box::new(Button {
                    width: 50,
                    height: 10,
                    label: "OK".to_string(),
                }),
            ],
        };

        screen.run();
    }
}
