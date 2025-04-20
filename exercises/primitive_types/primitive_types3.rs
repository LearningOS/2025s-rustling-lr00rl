// primitive_types3.rs
//
// Create an array with at least 100 elements in it where the ??? is.
//
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let a = [0; 1000];
    // In Rust, the [0; 1000] syntax is an array initialization expression that creates an array of 1000 elements, all set to the value 0.
    // In C code:
    // int a[1000] = {0}; // Initializes all elements to 0

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
