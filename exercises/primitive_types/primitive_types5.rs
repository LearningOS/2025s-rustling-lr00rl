// primitive_types5.rs
//
// Destructure the `cat` tuple so that the println will work.
//
// Execute `rustlings hint primitive_types5` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let cat = ("Furry McFurson", 3.5);  // tuple ?
    let /* your pattern here */(name, age) = cat;

    println!("{} is {} years old.", name, age);
}

/*
In Rust, tuples:

Are ordered collections of values of potentially different types
Have a fixed length determined at compile time
Use parentheses () syntax
Can be destructured using pattern matching, as shown in your example
Are indexed starting from 0, so you could also access elements with cat.0 and cat.1
*/
