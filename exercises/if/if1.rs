// if1.rs
//
// Execute `rustlings hint if1` or use the `hint` watch subcommand for a hint.

fn main() {
    println!("which bigger?");
    println!("bigger(10, 8) = {}", bigger(10, 8));
}

pub fn bigger(a: i32, b: i32) -> i32 {
    // Complete this function to return the bigger number!
    // Do not use:
    // - another function call
    // - additional variables

    if a > b {
        a
    } else {
        b
    }
}

// Don't mind this for now :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
        println!("Test ok ..")
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
        println!("Test ok ..")
    }
}

/*
Okay, let's break down that `#[cfg(test)]` block. You've correctly implemented the `bigger` function according to the constraints!

The block starting with `#[cfg(test)]` and containing `mod tests { ... }` is fundamental to how **unit testing** is commonly done in Rust.

Here's what each part means:

1.  **`#[cfg(test)]` - Conditional Compilation Attribute:**
    * `#[...]` denotes an **attribute** in Rust. Attributes provide metadata about pieces of code (like modules, functions, structs).
    * `cfg` stands for "configuration". This attribute enables **conditional compilation**.
    * `#[cfg(test)]` specifically tells the Rust compiler: "Only compile the code item that follows this attribute *if* the configuration flag `test` is currently active."
    * The `test` flag is automatically activated by Cargo (Rust's build tool and package manager) when you run the command `cargo test`.
    * When you run `cargo build` or `cargo run`, the `test` flag is *not* set, and therefore the entire `mod tests { ... }` block is completely ignored by the compiler – it doesn't end up in your final executable.

2.  **`mod tests { ... }` - The Test Module:**
    * `mod tests` declares a new **module** named `tests`. Modules are Rust's way of organizing code, controlling privacy, and creating namespaces.
    * It's a strong convention in Rust to put unit tests for the code in the current file (or current module) inside a submodule named `tests`. This keeps your tests co-located with the code they are testing but neatly separated.

3.  **`use super::*;` - Importing Items Under Test:**
    * This line is usually the first thing inside the `mod tests` block.
    * `super` refers to the **parent module** (the module *containing* `mod tests`). In your case, it's the main module defined by the file your `bigger` function is in.
    * `::*` is a glob import, meaning "import everything that is public (`pub`) or visible within the parent module into the current scope (`tests` module)".
    * This allows you to directly call functions like `bigger()` within your test functions, instead of having to write `super::bigger()`.

4.  **`#[test]` - Test Function Attribute:**
    * Inside the `mod tests` module, you see functions annotated with `#[test]`.
    * This attribute marks a function as a **test function**.
    * The test runner built into Rust (invoked by `cargo test`) specifically looks for functions marked with `#[test]` within code compiled under `cfg(test)`. It then executes each of these functions.
    * Test functions typically take no arguments and return nothing (`()`). (They can also return `Result<(), E>` to indicate test failure more granularly).

5.  **`assert_eq!(...)` - Assertion Macro:**
    * Inside a test function, you need to check if your code behaves correctly. Rust provides several **assertion macros** for this.
    * `assert_eq!(left, right)` checks if the value of the `left` expression is equal (`==`) to the value of the `right` expression.
    * If they are equal, the assertion passes, and the test continues.
    * If they are *not* equal, the `assert_eq!` macro will **panic**. The test runner catches this panic, marks the specific test function (`ten_is_bigger_than_eight` or `fortytwo_is_bigger_than_thirtytwo`) as **failed**, and usually prints the differing left and right values to help you debug. Other common macros include `assert!(expression)` (panics if `expression` is false) and `assert_ne!(left, right)` (panics if `left == right`).

**Why is this setup used?**

* **Organization:** Keeps tests close to the code they test.
* **No Runtime Cost:** Test code isn't included in your final compiled program unless you're specifically running tests. This keeps your production binaries smaller and potentially faster to compile.
* **Automation:** `cargo test` automatically discovers and runs all your tests.
* **Regression Prevention:** Tests ensure that future code changes don't accidentally break existing functionality that was previously working (this is called preventing "regressions").

**In summary:**

The `#[cfg(test)] mod tests { ... }` block is a standard Rust pattern for defining a separate module that contains unit tests. This module is only compiled when running `cargo test`. Inside this module, functions marked `#[test]` are executed by the test runner, and assertion macros like `assert_eq!` are used to verify that the code under test behaves as expected.
*/
