// if2.rs
//
// Step 1: Make me compile!
// Step 2: Get the bar_for_fuzz and default_to_baz tests passing!
//
// Execute `rustlings hint if2` or use the `hint` watch subcommand for a hint.


pub fn foo_if_fizz(fizzish: &str) -> &str {
/*
     In Rust, `&str` is a string slice - a reference to a part of a string stored somewhere else. It's used here for several good reasons:
     1. **Efficiency**: Using `&str` avoids copying the string data. Instead, it just borrows a reference to the existing string, which is more memory-efficient and faster.
     2. **Lifetimes**: The function both takes and returns a string slice. The input (`fizzish`) is borrowed from somewhere else, and the output (`"foo"`, `"bar"`, or `"baz"`) is a string literal that has a static lifetime (exists for the entire program).
     3. **Ownership**: By using `&str` instead of `String`, the function doesn't take ownership of the input string and doesn't allocate memory for the output - the string literals are stored in the program's binary.
     The function signature:
     ```rust
     pub fn foo_if_fizz(fizzish: &str) -> &str
     ```
     Tells us that:
     - It takes a borrowed string slice as input
     - It returns a borrowed string slice as output
     - The function doesn't own either the input or output strings
    This pattern is common in Rust for functions that need to examine string content but don't need to modify it or own it.

    In this situation you cannot modify the value. When you use `&str`, you're borrowing a reference to the string data, and Rust's borrowing rules prevent you from modifying borrowed data that's passed as an immutable reference.
    Key points about `&str`:
    1. It's an immutable reference to string data
    2. The `&` without `mut` indicates it's a read-only borrow
    3. The function isn't taking ownership of the string
    If you wanted to modify the string, you would need to use:
    - `&mut str` - A mutable string slice (rare in practice)
    - `String` - An owned, heap-allocated string that can be modified
    In this specific function, there's no need to modify the input string since it's just comparing values and returning different string literals based on those comparisons.
    If you needed to modify strings, the function might look more like:
    ```rust
    pub fn modify_string(input: &mut String) {
        input.push_str(" modified");
    }
    ```
    Or if you needed to create a new modified string:
    ```rust
    pub fn transform_string(input: &str) -> String {
        format!("{} transformed", input)
    }
```
*/
    if fizzish == "fizz" {
        "foo"
    } else if fizzish == "fuzz" {
        "bar"
    } else {
        "baz"
    }
}

// No test changes needed!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo_for_fizz() {
        assert_eq!(foo_if_fizz("fizz"), "foo")
    }

    #[test]
    fn bar_for_fuzz() {
        assert_eq!(foo_if_fizz("fuzz"), "bar")
    }

    #[test]
    fn default_to_baz() {
        assert_eq!(foo_if_fizz("literally anything"), "baz")
    }
}

/*
这一段代码是 Rust 的测试模块，我来详细解释一下它的用法和语法：

这段代码的主要组成部分：

1. `#[cfg(test)]` - 这是一个配置属性，表示这个模块只在运行测试时才会被编译。在正常构建程序时，这个模块会被忽略。

2. `mod tests` - 定义了一个名为 "tests" 的模块，用于组织测试代码。

3. `use super::*;` - 导入上级模块（包含 `foo_if_fizz` 函数的模块）中的所有内容，使得测试模块可以访问 `foo_if_fizz` 函数。

4. `#[test]` - 这是一个属性宏，标记下面的函数为一个测试函数。Rust 的测试运行器会找到并执行所有带有这个属性的函数。

5. 三个测试函数：
   - `foo_for_fizz()` - 测试当输入是 "fizz" 时函数返回 "foo"
   - `bar_for_fuzz()` - 测试当输入是 "fuzz" 时函数返回 "bar"
   - `default_to_baz()` - 测试当输入是其他任何字符串时函数返回 "baz"

6. `assert_eq!(a, b)` - 这是一个断言宏，检查两个值是否相等。如果不相等，测试会失败并显示错误信息。

这段代码的作用是确保 `foo_if_fizz` 函数按照预期工作：
- 当输入 "fizz" 时返回 "foo"
- 当输入 "fuzz" 时返回 "bar"
- 当输入其他任何内容时返回 "baz"

当你运行 `cargo test` 或者 `rustlings run` 命令时，Rust 会执行这些测试函数并报告它们是否通过。
*/
