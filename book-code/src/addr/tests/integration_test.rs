use addr;
mod common;

#[test]
fn it_adds_two() {
    // for i in 0..10 {
    //     //assert_eq!(i + 2, chapter_11::add_two(i));
    //     assert_eq!(i + 2, add_two(i));
    // }
    assert_eq!(4, addr::add_two(2));
}
