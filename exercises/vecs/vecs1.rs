// vecs1.rs
//
// Your task is to create a `Vec` which holds the exact same elements as in the
// array `a`.
//
// Make me compile and pass the test!
//
// Execute `rustlings hint vecs1` or use the `hint` watch subcommand for a hint.


fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // a plain array
    let v = vec![10, 20, 30, 40];// TODO: declare your vector here with the macro for vectors
                                 // NOTE: it is the macro `vec!` that you need to use

    (a, v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, v[..]);
    }
}

/*
I'd be happy to explain the differences between lists, tuples, and vectors in Rust!

In Rust, these three collection types serve different purposes and have distinct characteristics:

## Vec (Vector)

Vectors are the most commonly used collection in Rust. They're dynamic arrays that can grow or shrink in size.

Key characteristics:
- Mutable size (can add or remove elements)
- Stored on the heap
- Access elements by index in O(1) time
- Contiguous memory allocation
- Declared using `Vec<T>` where T is the type

Example:
```rust
let mut numbers: Vec<i32> = Vec::new();
numbers.push(1);
numbers.push(2);

// Or using the vec! macro
let numbers = vec![1, 2, 3];
```

## Tuple

Tuples are fixed-size collections of values that can have different types.

Key characteristics:
- Fixed size (determined at compile time)
- Can contain elements of different types
- Access elements using dot notation and index (.0, .1, etc.)
- Stored on the stack (unless they contain heap-allocated data)

Example:
```rust
let person: (String, i32, bool) = (String::from("Alice"), 30, true);
let name = person.0;
let age = person.1;
```

## Arrays (Lists)

In Rust, the term "list" typically refers to arrays, which are fixed-size collections of the same type.

Key characteristics:
- Fixed size (determined at compile time)
- All elements must be of the same type
- Access elements by index in O(1) time
- Stored on the stack
- Declared using `[T; N]` where T is the type and N is the length

Example:
```rust
let numbers: [i32; 3] = [1, 2, 3];
let first = numbers[0];
```

## Main Differences

1. **Mutability and size**:
   - Vec: Dynamic size, can grow and shrink
   - Tuple: Fixed size at compile time
   - Array: Fixed size at compile time

2. **Types of elements**:
   - Vec: All elements must be the same type
   - Tuple: Elements can be different types
   - Array: All elements must be the same type

3. **Memory allocation**:
   - Vec: Heap-allocated
   - Tuple: Stack-allocated (unless they contain heap data)
   - Array: Stack-allocated (unless they contain heap data or are very large)

4. **Access pattern**:
   - Vec: Index-based access with []
   - Tuple: Dot notation (.0, .1, etc.)
   - Array: Index-based access with []

Would you like me to elaborate on any specific aspect of these collection types?
*/
