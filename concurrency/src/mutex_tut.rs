use std::sync::Mutex;

fn main() {
    //..
}
fn sync_inc(counter: &Mutex<i32>) {
    let mut data: Guard<i32> = counter.lock();
    *data += 1;
} // unlock once Guard and Mutext falls out of scopre here
