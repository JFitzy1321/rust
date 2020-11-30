mod hashmap;
mod strings;
mod vectors;

fn main() {
    // vectors::main();
    strings::main();
    for b in "hello, fuckwad".bytes() {
        println!("{}", b);
    }

    hashmap::main();
}
