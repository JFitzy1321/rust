#[allow(dead_code)]

/// Chapter 3: Common Programming Concepts
pub fn main() {
    println!("Chapter 3: Variables!");
    variables();

    println!("Chapter 3: Shadowing!");
    shadowing();

    println!("Chapter 3: Array indexing");
    out_of_bounds_array();
    let num = 43;
    let num_type = ternary_num_type(&num);
    println!("Number is {}", num_type);
    for_loop_iterator();
    for_range();
}
pub fn variables() {
    let x = 5;
    println!("X is: {}", x);
    let x = 6;
    println!("X is now {} after shadowing!", x);
    // const MAX_INT: u32 = 100_000;
}

pub fn shadowing() {
    let text = "I am some text";
    println!("{}", text);
    let text = 32_876;
    println!("{}", text);
}

pub fn out_of_bounds_array() {
    let num_array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let index = 20;

    println!("The {} element is: {}", index, num_array[index]);
}

pub fn ternary_num_type<'a>(num: &'a u32) -> &'a str {
    // how todo a ternary operator in rust
    if num % 2 == 0 {
        "even"
    } else {
        "odd "
    }
}

fn for_loop_iterator() {
    let a = [10, 20, 30, 40, 50];

    for elem in a.iter() {
        println!("{}", elem);
    }
}

fn for_range() {
    for num in (1..5).rev() {
        println!("{}!", num);
    }
    println!("BLAST OFF!!!");
}
