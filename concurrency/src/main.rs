// fn main() {
//     let mut v = Vec::new();
//     push(&mut v);
//     read(&v);
// }

// // fn take(v: Vec<i32>) {
// //     //...
// // }
// fn push(v: &mut Vec<i32>) {
//     v.push(1);
// }

// fn read(v: &Vec<i32>) {
//     println!("{:?}", v);
// }

// use std::thread;

// fn main() {
//     let mut dst: Vec<i32> = Vec::new();
//     let loc = thread::spawn(move || dst.push(3));
//     //println!("Hello {}", loc.join().unwrap());
//     dst.push(4); // use after move error
// }

// // Shared reference between threads
// use std::sync::Arc;
// use std::thread;

// fn main() {
//     let v = Arc::new(vec![1, 2]);
//     let v2 = v.clone();
//     thread::spawn(move || {
//         println!("{}", v.len());
//     });
//     another_function(&v2);
// }

// fn another_function(_v: &Vec<i32>) {
//     //...
// }

mod my_object;

fn main() {
    let mut collection = my_object::AveragedCollection::new();

    collection.add(2);
    collection.add(5);
    collection.add(8);
    println!("Current average: {}", collection.average());
    match collection.remove() {
        Some(value) => println!("Value removed from collection: {}", value),
        None => println!("Nothing removed!"),
    };
    println!("New average: {}", collection.average());
}
