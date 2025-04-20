// primitive_types4.rs
//
// Get a slice out of Array a where the ??? is so that the test passes.
//
// Execute `rustlings hint primitive_types4` or use the `hint` watch subcommand
// for a hint.


#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    let nice_slice = &a[1..4];
    // &a[1..4] 代表从索引 1 开始，到索引 4 之前结束的切片

    assert_eq!([2, 3, 4], nice_slice)
}


/*
如果文件中只有这段测试代码，这是一个独立的 Rust 测试文件。让我来解释这段代码：

```rust
#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];
    let nice_slice = &a[1..4];
    assert_eq!([2, 3, 4], nice_slice)
}
```

这是一个测试函数，用于测试 Rust 的数组切片功能：

1. `#[test]` 是一个属性标记，表明这是一个测试函数。

2. `fn slice_out_of_array()` 定义了测试函数的名称。

3. `let a = [1, 2, 3, 4, 5];` 创建一个包含 5 个整数的数组。

4. `let nice_slice = &a[1..4];` 创建一个数组切片：
   - `&a` 表示对数组 `a` 的引用
   - `[1..4]` 表示从索引 1（第二个元素）开始，到索引 4（第五个元素）之前结束
   - 所以 `nice_slice` 是一个包含原数组中索引 1、2、3 位置元素的切片

5. `assert_eq!([2, 3, 4], nice_slice)` 断言 `nice_slice` 等于 `[2, 3, 4]`，验证切片操作是否正确。

如果这是文件中唯一的代码，你可以通过 `cargo test` 或 `rustc --test filename.rs` 来运行它。但注意这个文件缺少 `mod tests` 和必要的导入语句，在实际使用中可能需要添加这些内容：

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];
        let nice_slice = &a[1..4];
        assert_eq!([2, 3, 4], nice_slice)
    }
}
```
*/
