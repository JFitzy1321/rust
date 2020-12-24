#![warn(unused_variables)]

use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Data {
    Int(i32),
    Text(String),
    Float(f32),
}

pub fn main() {
    // let mut scores = HashMap::new();

    // scores.insert("Blue".to_string(), 10);
    // scores.insert("Yellow".to_string(), 50);

    //     let teams = vec!["Blue".to_string(), "Yellow".to_string()];
    //     let initial_scores = vec![10, 50];

    //     let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    //     println!("{:?}", scores);
    // let (field_name, field_value) = ("Favorite Color".to_string(), Data::Text("Red".to_string()));

    // let (fav_num, num) = ("Favorite Number".to_string(), Data::Int(42));
    // let mut map = HashMap::new();
    // map.insert(&field_name, field_value);
    // map.insert(&fav_num, num);
    // println!("{:?}", map);
    // // println!("Key: {}", field_name);
    // // i32, u32, and anything that implements Copy trait can be reused
    // // after inserting into HashMap
    // // println!("Favorite Number: {}", num.to_string());

    // let some_num = map.get(&fav_num).expect("Error trying to get value");
    // println!("Number from key {}: {:?}", &fav_num, some_num);
    // map.insert(&fav_num, Data::Float(3.14));

    // let fav_drink = "Favorite Drink".to_string();
    // map.entry(&fav_drink)
    //     .or_insert(Data::Text("Fat Tire Beer".to_string()));
    // println!("KV Pairs in map: ");
    // for (k, v) in &map {
    //     println!("{}: {:?}", k, v);
    // }
    let text = "hello fucking world and what a world it is";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
