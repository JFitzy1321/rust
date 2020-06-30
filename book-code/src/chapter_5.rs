struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    pub fn new(username: String, email: String) -> User {
        User {
            username,
            email,
            sign_in_count: 1,
            active: true,
        }
    }
}

pub fn main() {
    let user1 = User::new("someusername".to_string(), "someemail@com".to_string());

    // Struct Update Syntax
    let user2 = User {
        email: "anotheremail@com".to_string(),
        username: "anotheruser".to_string(),
        ..user1
    };

    struct Color(u32, u32, u32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    pub fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}
