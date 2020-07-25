// // /// Experimenting with enum and struct combos
// // enum IpAddr {
// //     V4(Ipv4Addr),
// //     V6(Ipv6Addr),
// // }

// // pub struct Ipv4Addr {
// //     address: String,
// // }

// // pub struct Ipv6Addr {
// //     address: String,
// // }

// // struct URL {
// //     ip: IpAddr,
// //     domain_name: String,

// // }

// // pub fn main() {
// //     let home = MyIpAddr {
// //         kind: IpAddrKind::V4,
// //         address: "127.0.0.1".to_string(),
// //     };
// // }

// // pub fn main() {
// //     enum IpAdd {
// //         V4(u8, u8, u8, u8),
// //         V6(String),
// //     }
// // }

// // enum Message {
// //     Quit,
// //     Move { x: i32, y: i32 },
// //     Write(String),
// //     ChangeColor(i32, i32, i32),
// // }

// // impl Message {
// //     fn call(&self) {
// //         // /// Not Sure about this
// //         // match *self {
// //         //     Quit => {}
// //         //     Move => {}
// //         //     Write => {}
// //         //     ChangeColor => {}
// //         // };
// //     }
// // }

// #[derive(Copy, Clone, Debug, Hash)]
// pub enum UsState {
//     Alabama,
//     Alaska,
//     Arizona,
//     Arkansas,
//     California,
//     Colorado,
//     Conneticut,
//     Delaware,
//     Florida,
//     Gerogia,
//     Idaho,
//     Illinois,
//     Indiana,
//     Iowa,
//     Kansas,
//     Kentucky,
//     Louisiana,
//     Maine,
//     Maryland,
//     Massachusetts,
//     Michigan,
//     Minnesota,
//     Mississippi,
//     Missouri,
//     Montana,
//     NorthCarolina,
//     NorthDakota,
//     Nebraska,
//     Nevada,
//     NewHemshire,
//     NewJersey,
//     NewMexico,
//     NewYork,
//     Ohio,
//     Oklahoma,
//     Orgeon,
//     Pennsylvania,
//     RhodeIsland,
//     SouthCarolina,
//     SouthDakota,
//     Tennesse,
//     Texas,
//     Utah,
//     Vermont,
//     Virginia,
//     Washington,
//     WestVirginia,
//     Wisconsin,
//     Wyoming,
// }

// #[derive(Copy, Clone, Debug, Hash)]
// pub enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }

// impl Coin {
//     pub fn value(&self) -> u8 {
//         match *self {
//             Coin::Penny => 1,
//             Coin::Nickel => 5,
//             Coin::Dime => 10,
//             Coin::Quarter(ref state) => {
//                 println!("State quarter from {:?}", state);
//                 25
//             }
//         }
//     }

//     pub fn get_state(&self) -> Option<&UsState> {
//         match *self {
//             Coin::Quarter(ref state) => Some(state),
//             _ => None,
//         }
//     }
// }
