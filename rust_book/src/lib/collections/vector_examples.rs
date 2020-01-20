// pub fn main() {
//     let v = vec![1, 2, 3, 4, 5];

//     let third: &i32 = &v[2];
//     println!("The third element (via indexing) is {}", third);

//     match v.get(2) {
//         Some(third) => println!("The third element (via indexing) is {}", third),
//         None => println!("There is no third element"),
//     };
// }

pub fn main() {
    // use enum to store multiple types in a vector

    let mut my_vec: Vec<MyType> = vec![];
    my_vec.push(MyType::SomeInt(123));
    my_vec.push(MyType::SomeString("Some random information".to_string()));
    my_vec.push(MyType::SomeFloat(13.59));
    my_vec.push(MyType::SomeDouble(12344.098123));
}

pub enum MyType {
    SomeInt(i32),
    SomeString(String),
    SomeFloat(f32),
    SomeDouble(f64),
}
