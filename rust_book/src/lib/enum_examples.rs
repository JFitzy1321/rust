pub fn main() {
    let _home = IpAddrKind::V4(127, 0, 0, 1);
    let _loopback = IpAddrKind::V6(String::from("::1"));
}

enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// impl IpAddr {
//     fn new(kind: IpAddrKind, address: String) -> {
//         IpAddr {
//             kind,
//             address,
//         }
//     }
// }

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method bod would be defined here
    }
}
