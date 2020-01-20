// Path to Material: file:///~/.rustup/toolchains/stable-x86_64-pc-windows-msvc/share/doc/rust/html/rust-by-example/flow_control/if_let.html
enum Foo {
    Bar,
    Baz,
    Qux(i32),
}

pub fn main() {
    let a: Foo = return_foo();
    if let Foo::Bar = a {
        println!("a is foobar!");
    }

    book_code();
}

fn return_foo() -> Foo {
    Foo::Bar
}

fn book_code() {
    let number = Some(70);
    let letter: Option<char> = None;
    let emoticon: Option<i32> = None;

    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }
}
