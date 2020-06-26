/// This is a dummy code base sololy for my educational purposes
/// Just like in C-style languages, Rust's default entry point
/// is the "main" function.
/// P.S. triple dashed comments is a doc comment
// how to "import" another  rust module in the same folder
// mod if_let_syntax_example;

// fn main() {
//     // Standard comment
//     // marcro call ( [macro_name]!() )
//     println!("Hello, world!");
//     if_let_syntax_example::main();
// }
mod hashset_sanbox;

fn main() {
    hashset_sanbox::main();
}
