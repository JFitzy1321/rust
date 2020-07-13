// /// Experimenting with enum and struct combos
// enum IpAddr {
//     V4(Ipv4Addr),
//     V6(Ipv6Addr),
// }

// pub struct Ipv4Addr {
//     address: String,
// }

// pub struct Ipv6Addr {
//     address: String,
// }

// struct URL {
//     ip: IpAddr,
//     domain_name: String,

// }

// pub fn main() {
//     let home = MyIpAddr {
//         kind: IpAddrKind::V4,
//         address: "127.0.0.1".to_string(),
//     };
// }

// pub fn main() {
//     enum IpAdd {
//         V4(u8, u8, u8, u8),
//         V6(String),
//     }
// }

// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// impl Message {
//     fn call(&self) {
//         // /// Not Sure about this
//         // match *self {
//         //     Quit => {}
//         //     Move => {}
//         //     Write => {}
//         //     ChangeColor => {}
//         // };
//     }
// }

pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

impl Coin {
    pub fn value(&self) -> u8 {
        match self {
            Penny => 1,
            Nickel => 5,
            Dime => 10,
            Quarter => 25,
        }
    }
}
