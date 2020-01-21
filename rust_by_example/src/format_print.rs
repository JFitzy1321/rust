/// Path to Rust By Example Section

fn main() {
    // Standard comment
    // marcro call ( [macro_name]!() )
    println!("Hello, world!");

    println!("{subject:?} {verb} {object}",
                subject = "Alice",
                verb = "In",
                object = "Chains"
            );
    // format number to binary
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

    // right pad a string with zeros
    println!("{number:>0width$}", number = 12445, width = 10);

}