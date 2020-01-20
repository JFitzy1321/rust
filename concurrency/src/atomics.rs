use std::sync::atomic::*;

fn main() {
    let number = AtomicUsize::new(10);
    let prev = number.fetch_add(1, SeqCst);
    assert_eq!(prev, 10);
    let prev = number.swap(2, SeqCst);
    assert_eq!(prev, 11);
    assert_eq!(number.load(SeqCst), 2);
}

struct Event {
    key: u32,
    count: u32,
    name: InternedString,
}
