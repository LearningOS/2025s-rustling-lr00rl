// move_semantics2.rs
//
// Expected output:
// vec0 has length 3, with contents `[22, 44, 66]`
// vec1 has length 4, with contents `[22, 44, 66, 88]`
//
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let vec0 = &mut Vec::<i32>::new();
    // let vec0 = &mut Vec::new();
    // let mut vec0 = &mut Vec::new();
/*
您的理解是正确的。关于 `mut` 关键字在这种情况下的使用：

在 `let vec0 = &mut Vec::new();` 和 `let mut vec0 = &mut Vec::new();` 这两种写法中，区别在于变量 `vec0` 本身是否可变，而非它所指向的内容。

1. 不加 `mut`（`let vec0 = &mut Vec::new();`）：
   - `vec0` 是一个不可变的变量，但它存储的是一个可变引用
   - 您可以通过这个引用修改向量的内容（如 `vec0.push(10)`）
   - 但您不能将 `vec0` 重新赋值为另一个引用

2. 加 `mut`（`let mut vec0 = &mut Vec::new();`）：
   - `vec0` 是一个可变的变量，存储的也是一个可变引用
   - 您既可以修改向量的内容
   - 也可以将 `vec0` 重新指向其他向量，例如：
     ```rust
     let mut vec0 = &mut Vec::new();
     vec0.push(1); // 修改向量内容
     
     let mut another_vec = Vec::new();
     vec0 = &mut another_vec; // 重新赋值 vec0 变量本身
     ```

所以，如果您只需要修改向量的内容而不需要重新赋值 `vec0` 变量本身，两种写法在功能上是等价的。但如果您需要重新赋值 `vec0`，则必须使用第二种带 `mut` 的写法。
*/

    let mut vec1 = fill_vec(vec0);

    println!("{} has length {}, with contents: `{:?}`", "vec0", vec0.len(), vec0); // NOTE:
                                                                                   // here used
                                                                                   // vec0

    vec1.push(88);

    println!("{} has length {}, with contents `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: &mut Vec<i32>) -> Vec<i32> {
    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec.to_vec()
}




/*
在 Rust 中，定义函数时参数的引用语法和调用函数时传递引用的语法确实需要同时使用 `&`，但它们的含义略有不同：

### 函数定义中的引用

当你定义一个接受引用的函数时，你在参数类型前添加 `&`：

```rust
// 接受一个 Vec<i32> 的不可变引用
fn example_function(vec: &Vec<i32>) { ... }

// 接受一个 Vec<i32> 的可变引用
fn example_function_mut(vec: &mut Vec<i32>) { ... }
```

这告诉编译器该函数希望借用传入的值，而不是获取其所有权。

### 函数调用中的引用

当你调用一个接受引用的函数时，你也需要在参数前使用 `&` 来创建引用：

```rust
let vec1 = Vec::new();
example_function(&vec1);  // 创建并传递一个不可变引用

let mut vec2 = Vec::new();
example_function_mut(&mut vec2);  // 创建并传递一个可变引用
```

### 两者需要匹配

在 Rust 中，函数签名和函数调用需要匹配：

- 如果函数定义为接受 `&Vec<i32>`，你必须传递 `&vec`
- 如果函数定义为接受 `&mut Vec<i32>`，你必须传递 `&mut vec`
- 如果函数定义为接受 `Vec<i32>`（值而非引用），你必须传递 `vec`（或 `vec.clone()`，如果你想保留原始值）

这是 Rust 类型系统的一部分，确保函数接收到它期望的类型，并且明确表示什么时候发生所有权转移，什么时候只是借用。


在 Rust 中，`fn example_function(&vec: &Vec<i32>) { ... }` 这样的语法是不合法的。

函数参数的声明格式是：`参数名: 类型`。在这个格式中，参数名前面不应该有 `&` 符号。`&` 符号只应出现在类型的部分，表示这个参数是一个引用类型。

正确的函数定义应该是：

```rust
fn example_function(vec: &Vec<i32>) { ... }
```

其中，`vec` 是参数名，`&Vec<i32>` 是类型（表示这是一个对 `Vec<i32>` 的不可变引用）。

如果你在函数定义中写 `&vec: &Vec<i32>`，编译器会产生语法错误，因为它把第一个 `&` 视为参数名称的一部分，这在 Rust 的语法中是不允许的。
*/










/*
fn main() {
    let mut vec0 = Vec::new();
    let mut vec1 = fill_vec(vec0);
    println!("{} has length {}, with contents: {:?}", "vec0", vec0.len(), vec0); // NOTE:
                                                                                   // here used
                                                                                   // vec0
    vec1.push(88);
    println!("{} has length {}, with contents {:?}", "vec1", vec1.len(), vec1);
}
fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(22);
    vec.push(44);
    vec.push(66);
    vec.to_vec()
}

⚠️  Compilation of exercises/move_semantics/move_semantics2.rs failed!, Compiler error message:
warning: variable does not need to be mutable
  --> exercises/move_semantics/move_semantics2.rs:12:9
   |
12 |     let mut vec0 = Vec::new();
   |         ----^^^^
   |         |
   |         help: remove this mut
   |
   = note: #[warn(unused_mut)] on by default
error[E0382]: borrow of moved value: vec0
  --> exercises/move_semantics/move_semantics2.rs:42:65
   |
12 |     let mut vec0 = Vec::new();
   |         -------- move occurs because vec0 has type Vec<i32>, which does not implement the Copy trait
...
40 |     let mut vec1 = fill_vec(vec0);
   |                             ---- value moved here
41 |
42 |     println!("{} has length {}, with contents: {:?}", "vec0", vec0.len(), vec0); // NOTE:
   |                                                                 ^^^^ value borrowed here after move
   |
note: consider changing this parameter type in function fill_vec to borrow instead if owning the value isn't necessary
  --> exercises/move_semantics/move_semantics2.rs:51:22
   |
51 | fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
   |    --------          ^^^^^^^^ this parameter takes ownership of the value
   |    |
   |    in this function
help: consider cloning the value if the performance cost is acceptable
   |
40 |     let mut vec1 = fill_vec(vec0.clone());
   |                                 ++++++++
error: aborting due to 1 previous error; 1 warning emitted
For more information about this error, try rustc --explain E0382.

解释一下


这个编译错误主要是关于 Rust 的所有权（ownership）和移动语义（move semantics）的问题。让我解释一下：

1. 第一个警告是：
```
warning: variable does not need to be mutable
  --> exercises/move_semantics/move_semantics2.rs:12:9
   |
12 |     let mut vec0 = Vec::new();
   |         ----^^^^
```
这是因为 `vec0` 在创建后没有被修改（在它被传递到 `fill_vec` 之前），所以不需要声明为 `mut`。

2. 主要错误是：
```
error[E0382]: borrow of moved value: `vec0`
  --> exercises/move_semantics/move_semantics2.rs:42:65
```

这是因为当你调用 `fill_vec(vec0)` 时，`vec0` 的所有权被转移到了函数内部。在 Rust 中，当你将一个值传递给函数而不是引用时，默认情况下会发生所有权转移。由于 `Vec<i32>` 类型没有实现 `Copy` trait，当你将它传递给函数时，原始变量失去了所有权，不能再使用。

所以当你尝试在 `println!` 中使用 `vec0.len()` 和 `vec0` 时，编译器报错，因为 `vec0` 已经被移动（moved）了。

编译器提供了两种解决方案：

1. 修改 `fill_vec` 函数，让它接收引用而不是获取所有权：
```rust
fn fill_vec(vec: &mut Vec<i32>) -> Vec<i32> {
    vec.push(22);
    vec.push(44);
    vec.push(66);
    vec.clone() // 或使用 to_vec()
}
```

2. 使用 `.clone()` 创建一个副本传递给函数，保留原始 `vec0` 的所有权：
```rust
let mut vec1 = fill_vec(vec0.clone());
```

这就是 Rust 所有权系统的工作方式，它在编译时防止数据竞争和内存安全问题。
*/
