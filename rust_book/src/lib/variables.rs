//variables.rs
//learning about rust from the rust book [ rustup doc --book ]

pub fn run() {
    // let x = 13;
    // print_stuff(format!("The value of non-mutable x is {}", x));
    // let x = 19;
    // print_stuff(String::from(
    //     "Variable shadowing allows for this to work
    //     \n\" let x = 13; \n  let x = 19; \"",
    // ));
    // println!("The new value of non-mutable x is {}", x);

    let tup: (i32, String, f64) = (13, "cocksucker".to_string(), 3.14);

    println!("value of a tuple's first element: {:?}", tup.0);
    println!("value of tuple's second element: {}", tup.1);
    println!("value of a tuple's third element: {}", tup.2);
}

pub fn get_int_from_console() {
    use std::io;

    println!("Please enter a number");
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap(); //unwrap() ingnore Error and lets the exception be thrown
    let input: i32 = input
        .trim()
        .parse()
        .expect("You didn't enter a number, dumbass"); //expectt(string) will handle exceptions by showing the message you provide

    println!("Your number was: {}", input);
}
