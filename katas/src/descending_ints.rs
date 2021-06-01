use itertools::Itertools;

pub fn main(x: u64) -> u64 {
    x.to_string()
        .chars()
        .sorted()
        .rev()
        .collect::<String>()
        .parse::<u64>()
        .expect("Error thrown in descending_ints function!")
}

#[cfg(test)]
mod tests {
    use super::main;

    #[test]
    fn reverse_int_works() {
        assert_eq!(4321, main(1234));
    }
}
