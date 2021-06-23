mod fibonacci;

fn main() {
    calc_phi();
    fibonacci::main();
}

fn calc_phi() {
    let phi = (1.0 + (5.0f64).sqrt()) / 2.0;
    println!("PHI calculated in Rust: {}", phi);
}

