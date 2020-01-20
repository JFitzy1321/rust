// to use another function in a diffirent file in side the same folder, call the file with this
// mod [file_name]
// then call the function with
// file_name::function();
pub fn main() {
    let x = 21; // x can not be change because it is not mutable by default

    print_num(&x); // <- this creates a "read only pointer (or reference)" to x"

    println!("Value of X back in main function: {}", x);
}

//This function takes in a "read only" reference variable
// ANy changes made to this reference will not be saved once outside of this functions scope
fn print_num(x: &i32) {
    println!("Value of X coming in to function: {}", x);
    let y = squared(x);
    println!("Value of X after changing the reference: {}", y);
}

// functions do not need a return key word, but only for the last line in the function,
// and only for expressions (and with no ; ).
fn squared(i: &i32) -> i32 {
    //Even though i passed in a readonly reference, i can still perform actions with the value
    //inside the reference, i just can change the reference.
    //But i can store the value to the reference locally, and return the value of my operations
    let i = *i; // <- deference the pointer and shadowing the i variable
                // this line is kind of redudant apparently.AsMut
                //rust will multiple and return the value all in one "Expression"

    i * i // this is any expression. It results in a value. It also returns that value
          // because it is the last line of code in this function's scope
          // don't use ; it makes that line a statement, which has does not result in a value
}
