pub mod generics {
    // Code from the chapter synopsis
    // pub fn main() {
    //     let number_list = vec![34, 50, 25, 100, 65];

    //     // let mut largest = number_list[0];

    //     // for number in number_list {
    //     //     if number > largest {
    //     //         largest = number;
    //     //     }
    //     // }

    //     // println!("The largest number is {}", largest);

    //     let result = largest(&number_list);
    //     println!("The largest number is {}", result);

    //     let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    //     let result = largest(&number_list);
    //     println!("The largest number is {}", result);
    // }

    // fn largest(list: &[i32]) -> &i32 {
    //     let mut largest = &list[0];

    //     for item in list {
    //         if item > largest {
    //             largest = item;
    //         }
    //     }

    //     largest
    // }

    // fn largest_char(list: &[char]) -> &char {
    //     let mut largest = &list[0];

    //     for item in list {
    //         if item > largest {
    //             largest = item;
    //         }
    //     }

    //     largest
    // }
    #[allow(dead_code)]
    fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];

        for item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }
    #[allow(dead_code)]
    fn print_with_typename<T: std::fmt::Debug>(var: &T) {
        println!("The largest {} is {:?}", std::any::type_name::<T>(), var);
    }

    #[allow(dead_code)]
    pub fn main() {
        let number_list = vec![34, 50, 25, 100, 65];

        // let result = largest_i32(&number_list);
        let result = largest(&number_list);
        print_with_typename(result);

        let char_list = vec!['y', 'm', 'a', 'q'];

        // let result = largest_char(&char_list);
        let result = largest(&char_list);
        print_with_typename(result);
    }
}

pub mod lifetimes {
    // // Code doesn't compile because x is dropped,
    // // but r still has a reference to it
    // pub fn main() {
    //     let r;
    //     {
    //         let x = 5;
    //         r = &x;
    //     }

    //     println!("r: {}", r);
    // }

    // pub fn main() {
    //     let string1 = "abcd".to_string();
    //     let string2 = "xyz";

    //     println!(
    //         "The longest string is '{}'",
    //         longest(string1.as_str(), string2)
    //     );
    // }

    // // Code won't compile because result code have reference to string2,
    // // which code be referenced outside it's lifetime (scope in this case)
    // pub fn main() {
    //     let string1 = "abcd".to_string();
    //     let result;
    //     {
    //         let string2 = "xyz".to_string();
    //         result = longest(string1.as_str(), string2.as_str());
    //     }
    //     println!("The longest string is '{}'", result);
    // }
    #[allow(dead_code)]
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    #[allow(dead_code)]
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    impl ImportantExcerpt<'_> {
        #[allow(dead_code)]
        fn level(&self) -> i32 {
            42
        }

        #[allow(dead_code)]
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please! : {}", announcement);
            self.part
        }
    }

    #[allow(dead_code)]
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }

    #[allow(dead_code)]
    pub fn main() {
        let novel = "Call me Ishmael. Some years ago...";
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        let _i = ImportantExcerpt {
            part: first_sentence,
        };
    }

    use std::fmt::Display;

    #[allow(dead_code)]
    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}
