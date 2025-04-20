// functions4.rs
//
// This store is having a sale where if the price is an even number, you get 10
// Rustbucks off, but if it's an odd number, it's 3 Rustbucks off. (Don't worry
// about the function bodies themselves, we're only interested in the signatures
// for now. If anything, this is a good way to peek ahead to future exercises!)
//
// Execute `rustlings hint functions4` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let original_price = 51;
    println!("Your sale price is {}", sale_price(original_price));
}

fn sale_price(price: i32) -> i32 {
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}

fn is_even(num: i32) -> bool {
    num % 2 == 0
}


/*
You've hit on a key feature of Rust's design.

Rust is an **expression-based language**. This means that most things, including blocks of code like `if`/`else` branches and even entire function bodies, evaluate to a value.

Here's the rule:

1.  **Implicit Return:** If the **last line** of a function (or any block) is an **expression** (meaning it evaluates to a value) and it **does not** have a semicolon (`;`) at the end, that expression's value becomes the return value of the function (or block).

2.  **Explicit `return` Keyword:** Rust *does* have the `return` keyword, but it's typically only used for **early returns** – when you want to exit the function before reaching the last line.

Let's look at your `sale_price` function:

```rust
fn sale_price(price: i32) -> i32 {
    if is_even(price) {
        price - 10 // No semicolon: This expression's value is returned if true
    } else {
        price - 3  // No semicolon: This expression's value is returned if false
    }
    // The if/else block itself is an expression.
    // Since it's the last expression in the function, its value is returned.
}
```

And your `is_even` function:

```rust
fn is_even(num: i32) -> bool {
    num % 2 == 0 // No semicolon: This boolean expression's value is returned
}
```

In both cases, the final expression doesn't have a semicolon, so its value is implicitly returned, matching the function's return type signature (`-> i32` or `-> bool`).

You *could* write `sale_price` with `return`, but it's less idiomatic for the final value:

```rust
// Less idiomatic way for the final value
fn sale_price_explicit(price: i32) -> i32 {
    if is_even(price) {
        return price - 10; // Explicit return
    } else {
        return price - 3;  // Explicit return
    }
    // Note: If you use return in all paths, the compiler is happy.
    // If you only used return in one path, the other path would still
    // need to provide the return value implicitly (no semicolon) or explicitly.
}
```

An example where `return` is needed for an *early* exit:

```rust
fn find_first_even(numbers: &[i32]) -> Option<i32> {
    for &num in numbers {
        if is_even(num) {
            return Some(num); // Early return: Found the first even number
        }
    }
    None // Implicit return: No even number found after checking all
}
```

So, you are right! Rust often doesn't need the `return` keyword because the value of the final expression (without a semicolon) is automatically used as the return value.
*/
