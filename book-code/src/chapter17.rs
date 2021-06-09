// OOP in Rust
pub fn main() {
    trait_objects::main();

    oop_design::main();
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

    // Btw, Trait Objects must be 'Object Safe':
    // - The return type isn’t Self.
    // - There are no generic type parameters.
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

mod oop_design {

    // We can do better!

    // mod blog {
    //     trait State {
    //         fn request_review(self: Box<Self>) -> Box<dyn State>;
    //         fn approve(self: Box<Self>) -> Box<dyn State>;
    //         fn content<'a>(&self, _post: &'a Post) -> &'a str {
    //             ""
    //         }
    //         fn reject(self: Box<Self>) -> Box<dyn State>;
    //     }

    //     struct Draft {}

    //     impl State for Draft {
    //         fn request_review(self: Box<Self>) -> Box<dyn State> {
    //             Box::new(PendingReview { approvals: 0 })
    //         }

    //         fn approve(self: Box<Self>) -> Box<(dyn State + 'static)> {
    //             self
    //         }

    //         fn reject(self: Box<Self>) -> Box<dyn State> {
    //             self
    //         }
    //     }

    //     struct PendingReview {
    //         approvals: u8,
    //     }

    //     impl State for PendingReview {
    //         fn request_review(self: Box<Self>) -> Box<dyn State> {
    //             self
    //         }

    //         fn approve(self: Box<Self>) -> Box<dyn State> {
    //             if self.approvals < 2 {
    //                 let state_plus1 = PendingReview {
    //                     approvals: self.approvals + 1,
    //                 };
    //                 Box::new(state_plus1)
    //             } else {
    //                 Box::new(Published {})
    //             }
    //         }

    //         fn reject(self: Box<Self>) -> Box<dyn State> {
    //             Box::new(Draft {})
    //         }
    //     }

    //     struct Published {}

    //     impl State for Published {
    //         fn request_review(self: Box<Self>) -> Box<dyn State> {
    //             self
    //         }

    //         fn approve(self: Box<Self>) -> Box<dyn State> {
    //             self
    //         }

    //         fn content<'a>(&self, post: &'a Post) -> &'a str {
    //             &post.content
    //         }

    //         fn reject(self: Box<Self>) -> Box<dyn State> {
    //             self
    //         }
    //     }

    //     pub struct Post {
    //         state: Option<Box<dyn State>>,
    //         content: String,
    //     }

    //     impl Post {
    //         pub fn new() -> Self {
    //             Self {
    //                 state: Some(Box::new(Draft {})),
    //                 content: String::new(),
    //             }
    //         }

    //         pub fn add_text<T: AsRef<str>>(&mut self, text: T) {
    //             self.content.push_str(text.as_ref());
    //         }

    //         pub fn content(&self) -> &str {
    //             self.state.as_ref().unwrap().content(self)
    //         }

    //         pub fn request_review(&mut self) {
    //             if let Some(s) = self.state.take() {
    //                 self.state = Some(s.request_review())
    //             }
    //         }

    //         pub fn approve(&mut self) {
    //             if let Some(s) = self.state.take() {
    //                 self.state = Some(s.approve());
    //             }
    //         }
    //     }
    // }

    // use blog::Post;

    // pub fn main() {
    //     let mut post = Post::new();
    //     let post_text = "I ate a salad for lunch today".to_string();
    //     post.add_text(post_text.clone());
    //     assert_eq!("", post.content());

    //     post.request_review();
    //     assert_eq!("", post.content());

    //     post.approve();
    //     assert_eq!(&post_text, post.content());
    // }

    mod blog {
        pub struct Post {
            content: String,
        }

        pub struct DraftPost {
            content: String,
        }

        pub struct PendingReviewPost {
            content: String,
        }

        impl Post {
            pub fn new() -> DraftPost {
                DraftPost {
                    content: String::new(),
                }
            }

            pub fn content(&self) -> &str {
                &self.content
            }
        }

        impl DraftPost {
            pub fn add_text(&mut self, text: &str) {
                self.content.push_str(text);
            }

            pub fn request_review(self) -> PendingReviewPost {
                PendingReviewPost {
                    content: self.content,
                }
            }
        }

        impl PendingReviewPost {
            pub fn approve(self) -> Post {
                Post {
                    content: self.content,
                }
            }

            pub fn reject(self) -> DraftPost {
                DraftPost {
                    content: self.content,
                }
            }
        }
    }

    use blog::Post;

    pub fn main() {
        let mut post = Post::new();

        post.add_text("I ate a salad for lunch today");

        let post = post.request_review();

        let post = post.approve();

        assert_eq!("I ate a salad for lunch today", post.content());
    }
}
