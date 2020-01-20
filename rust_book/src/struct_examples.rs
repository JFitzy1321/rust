// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }

// struct Color(i32, i32, i32)
// struct Point(i32, i32, i32)


// pub fn main() {
//     let black = Color(0,0,0);

//     let user1 = User {
//         email: String::from("something@.com"),
//         username: String::from("username123"),
//         active: true,
//         sign_in_count: 1,
//     };

//     let user2 = User {
//         email: String::from("otheremail.com"),
//         username: String::from("username"),
//         ..user1
//     };
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn is_square(&self) -> bool {
        self.width == self.height
    }


    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size}
    }
}

pub fn main() {
    //let rect1 = Rectangle { width: 30, height: 50};

    // "static" method call
    // does not have 'self' param
    let rect1 = Rectangle::square(100);

    // uses debug formatter
    println!("rect1 is {:?}", rect1);

    // "instance" method call
    // takes in 'self' as param
    println!("The are of the rectangle is {} square pixels.", rect1.area());

    match rect1.is_square() {
        true => println!{"rect1 is also a square!"},
        false => println!("rect is not a square!")
    }

    let rect2 = Rectangle::new(10, 40);
    println!("rect2 is {:?}", rect2);

    let rect3 = Rectangle::new(60, 50);
    println!("rect3 is {:?}", rect3);

    println!("Can 'rect2' fit inside rect1? {}", bool_to_str(rect1.can_hold(&rect2)));
    println!("Can 'rect3' fit inside rect1? {}", bool_to_str(rect1.can_hold(&rect3)));
}

fn bool_to_str(thing: bool) -> String {
    match thing {
        true => String::from("yes"),
        false => String::from("no"),
    }
}
