// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!

// I AM NOT DONE

fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");
    string("red".to_string());
    string(String::from("hi"));
    string("rust is fun!".to_owned());
    string_slice("nice weather".into());
    string(format!("Interpolation {}", "Station"));
    string_slice(&String::from("abc")[0..1]);
    string_slice("  hello there ".trim());
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}


/*
I'd be happy to explain the Rust string operations functions and the distinction between `&str` and `String` in your code example.

## `&str` vs `String` in Rust

### `&str` (String Slice)
- A string slice is a reference to a portion of a string
- It's a borrowed type (doesn't own the data)
- It's immutable by default
- Very memory efficient as it doesn't allocate new memory
- Often used for function parameters to avoid unnecessary copying

### `String`
- An owned string type that allocates memory on the heap
- Mutable and can be modified
- Has ownership of its data
- Provides more functionality for manipulating strings
- Generally used when you need to own or modify string data

## String Operations in Your Example

Let's go through each line in the `main()` function:

1. `string_slice("blue");`
   - Passes a string literal directly to `string_slice`
   - String literals are `&str` by default in Rust

2. `string("red".to_string());`
   - `.to_string()` converts a string literal to an owned `String`
   - Creates a new `String` on the heap

3. `string(String::from("hi"));`
   - `String::from()` is another way to create a `String` from a string literal
   - Functionally equivalent to `.to_string()`

4. `string("rust is fun!".to_owned());`
   - `.to_owned()` is similar to `.to_string()`, creating an owned `String`
   - It's a more generic method as it works on more types than just strings

5. `string_slice("nice weather".into());`
   - `.into()` performs type conversion based on context
   - In this case, it keeps it as a `&str` because that's what `string_slice` expects

6. `string(format!("Interpolation {}", "Station"));`
   - `format!` macro creates a new `String` with interpolated values
   - Similar to `println!` but returns a `String` instead of printing

7. `string_slice(&String::from("abc")[0..1]);`
   - Creates a `String` from "abc"
   - Takes a slice of the first character (`[0..1]`)
   - The result is a `&str` slice referring to just "a"

8. `string_slice("  hello there ".trim());`
   - `.trim()` removes whitespace from both ends of a string
   - Returns a `&str` slice of the trimmed portion

9. `string("Happy Monday!".to_string().replace("Mon", "Tues"));`
   - Creates a `String` from the literal
   - `.replace()` creates a new `String` with "Mon" replaced by "Tues"
   - Result is "Happy Tuesday!"

10. `string("mY sHiFt KeY iS sTiCkY".to_lowercase());`
    - `.to_lowercase()` converts all characters to lowercase
    - Returns a new `String` with the transformed text

## Key Concepts

1. **Memory Management**:
   - `&str` is a borrowed view that doesn't own memory
   - `String` owns its memory on the heap

2. **Conversion Methods**:
   - `.to_string()`, `String::from()`, `.to_owned()` convert from `&str` to `String`
   - You can get a `&str` from a `String` with `&` operator or slicing

3. **String Manipulation**:
   - Most methods that transform strings (like `.replace()` or `.to_lowercase()`) return new `String` instances
   - Methods like `.trim()` return `&str` slices because they don't need to allocate new memory

This distinction helps Rust achieve memory safety without garbage collection while still providing efficient string operations.
*/
