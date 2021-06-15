pub fn main() {
    unsafe_rust::main();
}

mod unsafe_rust {
    // Unsafe code gives access to these five "unsafe superpowers"
    // - Dereference raw pointers
    //      -- *const T (immutable pointer)
    //      -- *mut T (mutable pointer)
    // - Call unsafe functions or methods
    // - Implement unsafe traits
    // - Access fields of `union`s (wtf is a union in rust)
    // It's also need for FFI (Foreign Function Interface) and accessing hardware
    // Because these things are 'outside' the scope of the compiler and borrow checker,
    // rust cannot guarantee saftey with this operations
    pub fn main() {
        let mut num = 5;

        // we can create raw pointers in 'safe' code
        // but we can't dereference them here
        let r1 = &num as *const i32;
        let r2 = &mut num as *mut i32;
        unsafe {
            println!("r1 is: {}", *r1);
            println!("r2 is: {}", *r2);

            dangerous();

            println!("Absolute value of -3, according to C Library: {}", abs(-3));
        }

        // raw pointer to arbitrary memory
        // let address = 0x012345usize;
        // let r = address as *const i32;
    }

    unsafe fn dangerous() {}

    use std::slice;

    fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = slice.len();
        let ptr = slice.as_mut_ptr();

        assert!(mid <= len);

        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }

    // Call a C ABI
    extern "C" {
        fn abs(input: i32) -> i32;
    }

    // write a funcion to be used by c
    // pub extern "C" fn call_from_c() {
    //     println!("Just called a Rust function from C!");
    // }

    unsafe trait Foo {}

    unsafe impl Foo for i32 {}
}
