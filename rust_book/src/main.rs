#![allow(dead_code)]

// mod enum_examples;
use std::error::Error;

mod lib;

// use bin::control_flow;
// use bin::fibonacci;
// use bin::functions;
// use bin::guessing_game;
use lib::if_let;
// use bin::matching_examples;
// use bin::options_examples;
// use bin::ownership;
// use bin::slice_example;
// use bin::struct_examples;
// use bin::variables;

fn main() -> Result<(), Box<dyn Error>> {
    // enum_examples::main();
    // enum_examples::main();
    // control_flow::main();
    // matching_examples::main();
    if_let::main();
    Ok(())
}
