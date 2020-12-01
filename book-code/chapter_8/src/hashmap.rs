use std::collections::HashMap;

pub fn main() {
    // let mut scores = HashMap::new();

    // scores.insert("Blue".to_string(), 10);
    // scores.insert("Yellow".to_string(), 50);

    //     let teams = vec!["Blue".to_string(), "Yellow".to_string()];
    //     let initial_scores = vec![10, 50];

    //     let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    //     println!("{:?}", scores);
    let (field_name, field_value) = ("Favorite Color".to_string(), 2);

    let (fav_num, num) = ("Favorite number".to_string(), 42);
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    map.insert(fav_num, num);
    println!("{:?}", map);
    // println!("Key: {}", field_name);
    println!("Favorite Number: {}", num.to_string())
    // TODO: on Hash Maps and Ownership Section: https://doc.rust-lang.org/stable/book/ch08-03-hash-maps.html
}
