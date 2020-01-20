#![allow(dead_code)]

// mod enum_examples;
use std::error::Error;

mod lib;

// use lib::control_flow;
// use lib::fibonacci;
// use lib::functions;
// use lib::guessing_game;
// use lib::if_let;
// use lib::matching_examples;
// use lib::options_examples;
// use lib::ownership;
// use lib::slice_example;
// use lib::struct_examples;
// use lib::variables;
use lib::collections::vector_examples;

fn main() -> Result<(), Box<dyn Error>> {
    // enum_examples::main();
    // enum_examples::main();
    // control_flow::main();
    // matching_examples::main();
    // if_let::main();
    vector_examples::main();
    Ok(())
}
