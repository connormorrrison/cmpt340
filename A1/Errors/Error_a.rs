// Error_a.rs
// a) A lexical error, detected by the scanner

fn main() {
    let x = 42$;  // '$' is not a valid char here
}