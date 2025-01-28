// Error_e.rs
// An error that violates Rust's language definition by attempting to
// create a situation where a variable could have two different types
// at the same time

fn main() {
    let mut x = 5;  // x is an i32

    // Attempt to make x be an i32 and a &str simultaneously
    let ptr = &x as *const i32;
    unsafe {
        *(ptr as *mut &str) = "hello";  // Attempt to treat same memory as different type

        // x would need to simultaneously be both an
        // i32 and a &str; this violates Rust's type language rules
        println!("As number: {}", x);
        println!("As string: {}", *(ptr as *const &str));
    }
}