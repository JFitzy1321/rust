#[derive(Debug)]
enum SpreadSheetCell {
    Int(i32),
    Float(f32),
    Text(String),
}

fn main() {
    // vectors();
    strings();
    for b in "hello, fuckwad".bytes() {
        println!("{}", b);
    }
}

fn vectors() {
    // Initialization without any values, must specify type!
    //let v: Vec<i32> = Vec::new();

    // array Initialization with vec! macro
    {
        let mut v = vec![1, 2, 3, 4, 5];
        v.push(6);
        v.push(7);
        // v.push("Hello"); // error, not the same types

        //S let outofbounds = &v[10]; // will throw runtime error

        // match v.get(10) {
        //     Some(t) => println!("The 11th element is: {}", t),
        //     None => println!("There is no 11th element!"),
        // }

        {
            let first = &v[0];
            println!("The first element is: {}", first);
        }
        v.push(8);

        for i in &v {
            println!("{}", i);
        }

        for i in &mut v {
            *i += 50
        }

        println!("{:?}", &v);
    }

    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text(String::from("Some text")),
        SpreadSheetCell::Float(10.12),
    ];
    println!("{:?}", &row);
}

fn strings() {
    let mut s = String::new();

    let data = "initial contents".to_string();

    let mut s = String::from("foo");
    s.push_str("bar");
}
