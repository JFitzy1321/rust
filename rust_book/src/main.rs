// mod enum_examples;
use std::error::Error;

//mod control_flow;
// mod fibonacci;
// mod functions;
// mod guessing_game;
// mod options_examples;
// mod ownership;
// mod slice_example;
// mod struct_examples;
// mod variables;
// mod matching_examples;
mod if_let;

fn main() -> Result<(), Box<dyn Error>> {
    // enum_examples::main();
    // enum_examples::main();
    // control_flow::main();
    // matching_examples::main();
    if_let::main();
    Ok(())
}
