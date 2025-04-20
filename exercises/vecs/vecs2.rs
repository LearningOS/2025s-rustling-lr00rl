// vecs2.rs
//
// A Vec of even numbers is given. Your task is to complete the loop so that
// each number in the Vec is multiplied by 2.
//
// Make me pass the test!
//
// Execute `rustlings hint vecs2` or use the `hint` watch subcommand for a hint.


fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
    for element in v.iter_mut() {
        // TODO: Fill this up so that each element in the Vec `v` is
        // multiplied by 2.
        *element *= 2;
    }

    // At this point, `v` should be equal to [4, 8, 12, 16, 20].
    v
}
/*
这个函数通过迭代和修改原始向量来实现每个元素乘以2：

1. fn vec_loop(mut v: Vec<i32>) -> Vec<i32> - 定义一个名为vec_loop的函数，它接收一个可变的整数向量作为参数，并返回同一个向量
2. mut v: Vec<i32> - 参数v被声明为可变的，这允许我们直接修改它
3. for element in v.iter_mut() - 使用iter_mut()方法获取向量中每个元素的可变引用
4. *element *= 2 - 使用解引用操作符*访问元素的实际值，并将其乘以2
5. 最后，返回修改后的向量v
*/
/*
关于 *element *= 2 中的 * 符号
在Rust中，*确实是解引用操作符，功能类似于C/C++中的指针解引用。但在这个上下文中，它有一个特定的用途：
element是一个&mut i32类型（可变引用），而不是直接的i32值。这是因为iter_mut()方法返回的是元素的可变引用。要实际修改引用指向的值，我们需要使用*解引用操作符。
所以*element *= 2的意思是：

使用*获取引用指向的实际值
将该值乘以2
将结果存回原始内存位置

这与指针操作非常相似，只是Rust中的引用比C/C++的指针更安全，有更严格的所有权和生命周期规则。
*/




fn vec_map(v: &Vec<i32>) -> Vec<i32> {
    v.iter().map(|element| {
        // TODO: Do the same thing as above - but instead of mutating the
        // Vec, you can just return the new number!
        element * 2
    }).collect()
}
/*
这个函数使用函数式编程风格，创建一个新的向量而不修改原始向量：

1. fn vec_map(v: &Vec<i32>) -> Vec<i32> - 定义一个名为vec_map的函数，它接收一个整数向量的引用，并返回一个新的向量
2. v.iter() - 获取向量元素的不可变迭代器
3. .map(|element| { element * 2 }) - 对每个元素应用一个闭包函数，将元素乘以2生成新的值
    |element| 是一个闭包参数，接收当前元素
    这里不需要*解引用操作符，因为我们只需要返回一个新值，而不是修改原值
4. .collect() - 将映射后的值收集到一个新的向量中并返回
*/
/*
## 关于闭包 `|element|` 

闭包（Closure）是Rust中的匿名函数，它可以捕获环境中的变量。`|element|`定义了闭包的参数列表，类似于函数的参数定义。

在`v.iter().map(|element| { element * 2 })`中：
- `|element|`表示这个闭包接收一个参数，命名为`element`
- 花括号内是闭包的函数体，返回`element * 2`

### 闭包的高级用法示例：

1. **捕获环境变量**：
```rust
let factor = 3;
let multiplied = vec![1, 2, 3].iter().map(|x| x * factor).collect::<Vec<i32>>();
// 结果: [3, 6, 9]
```
闭包捕获了外部变量`factor`。

2. **多参数闭包**：
```rust
let pairs = vec![(1, 2), (3, 4)];
pairs.iter().map(|(a, b)| a + b).collect::<Vec<i32>>();
// 结果: [3, 7]
```
使用模式匹配解构元组参数。

3. **作为函数参数传递**：
```rust
fn apply_twice<F>(f: F, x: i32) -> i32 
where F: Fn(i32) -> i32 
{
    f(f(x))
}

let double = |x| x * 2;
let result = apply_twice(double, 5);  // 结果: 20
```

4. **结合复杂迭代器链**：
```rust
let data = vec![1, 2, 3, 4, 5];
let sum_of_squares = data.iter()
    .filter(|&x| x % 2 == 0)     // 只保留偶数
    .map(|&x| x * x)            // 平方
    .fold(0, |acc, x| acc + x); // 求和
// 结果: 20 (= 2² + 4²)
```
>> 在 `.filter(|&x| x % 2 == 0)` 和 `.map(|&x| x * x)` 中，`|&x|` 的写法涉及到模式匹配和引用的解构。让我解释这个语法：
>> 
>> ## `|&x|` 语法解析
>> 
>> 当使用 `iter()` 方法时，它产生的是对集合元素的引用（即 `&T` 类型，在这个例子中是 `&i32`）。所以在闭包参数中：
>> 
>> - 写法 `|x|` 中的 `x` 类型是 `&i32`（对整数的引用）
>> - 写法 `|&x|` 中的 `x` 类型是 `i32`（直接是整数值）
>> 
>> `|&x|` 是一种模式匹配语法，它通过解构引用，让你可以直接使用值而不是引用。
>> 
>> ## 为什么不用 `*`
>> 
>> 虽然你也可以写成 `|x| *x % 2 == 0`，但这有几个区别：
>> 
>> 1. **语法简洁性**：`|&x|` 比 `|x| *x...` 更简洁，特别是当你多次使用这个值时
>> 2. **语义清晰性**：通过参数模式直接表明你想要值，而不是引用
>> 3. **编译优化**：编译器可能能够更好地优化模式匹配形式
>> 
>> ## 对比两种方式
>> 
>> ```rust
>> // 使用引用解构模式
>> data.iter().filter(|&x| x % 2 == 0)
>> 
>> // 使用解引用操作符
>> data.iter().filter(|x| *x % 2 == 0)
>> ```
>> 
>> 两种写法都有效，不同之处在于：
>> 
>> - 第一种模式（`|&x|`）在参数定义时就进行了解构
>> - 第二种模式（`|x| *x`）在闭包体内部进行解引用
>> 
>> 对于简单操作，效果基本相同，更多是风格选择。对于复杂闭包或多次使用值的情况，模式匹配方式（`|&x|`）通常更清晰和简洁。
>> 
>> 这是Rust模式匹配的强大特性之一，它不仅适用于闭包参数，也适用于函数参数、`match`表达式、`let`绑定等多种场景。

5. **使用move关键字转移所有权**：
```rust
let data = vec![1, 2, 3];
let consumer = move || {
    // 此闭包获取data的所有权
    println!("Consumed: {:?}", data);
};
// 这里不能再使用data，因为所有权已经转移到闭包
```

6. **闭包作为返回值**：
```rust
fn create_adder(addend: i32) -> impl Fn(i32) -> i32 {
    move |x| x + addend
}

let add_five = create_adder(5);
println!("{}", add_five(10));  // 输出: 15
```

闭包是Rust中非常强大的功能，特别是与迭代器结合使用时，可以写出简洁、高效且富有表现力的代码。
*/


// explains:
/*
我来详细解释您提出的每个语法问题：

## 1. `apply_twice<F>` 中的 `<F>` 语法

这是Rust的泛型语法。`<F>`声明了一个类型参数，代表一个泛型类型：

```rust
fn apply_twice<F>(f: F, x: i32) -> i32 
where F: Fn(i32) -> i32 
```

- `<F>` 定义了一个泛型类型参数F
- `where F: Fn(i32) -> i32` 是类型约束，指定F必须是一个接收i32并返回i32的函数类型
- 这种泛型让函数可以接受任何满足特定特征(trait)的类型，而不限于具体类型

## 2. `.fold(0, |acc, x| acc + x)` 语法解析

`fold`是一个迭代器方法，用于将一个集合"折叠"成单个值。它的完整语法是：

```rust
fold(初始值, |累加器, 当前元素| 计算表达式)
```

具体解释：
- `0` 是累加器的初始值（在这个例子中代表和的初始值为0）
- `|acc, x|` 定义了一个接收两个参数的闭包：
  - `acc` 是累加器，保存之前计算的结果
  - `x` 是当前正在处理的元素
- `acc + x` 是计算表达式，将当前元素添加到累加器中

工作原理：
1. 第一次迭代：`acc`是初始值0，`x`是第一个元素
2. 之后每次迭代：`acc`是上一次迭代的结果，`x`是当前元素
3. 最终返回累加器的最终值

例子：
```rust
let numbers = vec![1, 2, 3, 4];
let sum = numbers.iter().fold(0, |acc, x| acc + x);  // 结果: 10
```

`fold`不仅限于加法，可以用于任何运算：
```rust
// 计算乘积
let product = numbers.iter().fold(1, |acc, x| acc * x);  // 结果: 24

// 找最大值
let max = numbers.iter().fold(0, |acc, x| if x > &acc { *x } else { acc });

// 构建字符串
let words = vec!["Hello", "world"];
let sentence = words.iter().fold(String::new(), |mut acc, x| {
    if !acc.is_empty() { acc.push(' '); }
    acc.push_str(x);
    acc
});  // 结果: "Hello world"
```

## 3. `move` 关键字语法

`move` 关键字用于强制闭包获取其捕获变量的所有权，而不是借用它们：

```rust
let data = vec![1, 2, 3];
let consumer = move || {
    // 此闭包获取data的所有权
    println!("Consumed: {:?}", data);
};
```

语法：
- `move` 关键字放在闭包参数前
- `move || { ... }` 或 `move |params| { ... }`

用途：
1. **延长变量生命周期**：当闭包需要在捕获变量超出作用域后继续使用时
2. **线程间数据传递**：在多线程编程中，闭包需要拥有数据的所有权才能安全地发送到其他线程
3. **避免引用问题**：在某些情况下，编译器可能无法推断借用的生命周期

使用场景：
```rust
// 1. 线程中使用
let data = vec![1, 2, 3];
std::thread::spawn(move || {
    // 闭包获取data所有权并带入新线程
    println!("{:?}", data);
});

// 2. 返回闭包
fn create_counter() -> impl Fn() -> i32 {
    let mut count = 0;
    move || {
        count += 1;
        count
    }
}

// 3. 异步编程中
async fn process() {
    let resource = acquire_resource();
    tokio::spawn(async move {
        // 获取resource所有权
        use_resource(resource).await;
    });
}
```

## 4. `impl Fn(i32)` 语法

`impl Trait` 是Rust中一种返回类型语法，代表"实现了某个特定trait的类型"：

```rust
fn create_adder(addend: i32) -> impl Fn(i32) -> i32 {
    move |x| x + addend
}
```

语法解析：
- `impl Fn(i32) -> i32` 表示"一个实现了`Fn(i32) -> i32`特征的类型"
- `Fn(i32) -> i32` 是一个特征，描述了接收i32并返回i32的函数类型

优势：
1. **抽象类型**：不需要指定确切的闭包类型（这在Rust中很复杂）
2. **封装实现细节**：调用者只需知道返回的是一个函数，不需要知道具体类型
3. **简化复杂类型**：闭包的实际类型在Rust中通常很复杂，使用`impl`简化了语法

其他使用场景：
```rust
// 返回迭代器
fn create_sequence(n: i32) -> impl Iterator<Item = i32> {
    (1..=n).map(|x| x * x)
}

// 泛型条件返回
fn get_larger<T: PartialOrd>(a: T, b: T) -> impl PartialOrd {
    if a > b { a } else { b }
}

// 组合多个特征
fn create_processor() -> impl Fn(i32) -> i32 + Clone + Send {
    |x| x * 2
}
```

这些语法特性是Rust语言表达能力和类型系统的强大体现，它们使代码既安全又富有表现力。
*/

/*
# 理解 `move` 关键字

`move` 的核心作用是改变闭包捕获外部变量的方式 - 从默认的借用(借引用)变为获取所有权。

## 为什么例子都用 `move || {}` 格式

这个格式是Rust闭包的标准语法：
- `||` 是闭包的参数部分（类似函数的`()`）
- `{}` 是闭包的函数体
- `move` 放在参数之前，表示"获取所有捕获变量的所有权"

`move` 可以应用于任何闭包，不仅限于无参数闭包：
```rust
// 无参数闭包
let closure1 = move || { println!("I own data now"); };

// 有参数闭包
let closure2 = move |x: i32| { println!("I own data and received {}", x); };
```

## `move` 的实际作用示例

没有 `move` 时：
```rust
let data = String::from("hello");
let closure = || {
    // 默认情况下闭包借用data
    println!("{}", data);
};
// 这里仍然可以使用data，因为闭包只是借用了它
println!("原数据: {}", data);
closure();
```

使用 `move` 时：
```rust
let data = String::from("hello");
let closure = move || {
    // 闭包获取data的所有权
    println!("{}", data);
};
// 错误：data已经被移动到闭包中，无法再使用
// println!("原数据: {}", data); // 这行会导致编译错误
closure();
```

## 闭包和 Lambda 函数的区别

在Rust中，闭包和Lambda函数本质上是同一个概念，但有一些细微差别：

1. **环境捕获**：
   - 闭包可以捕获外部环境中的变量
   - 纯Lambda函数在某些语言中可能仅限于其参数

2. **术语来源**：
   - "闭包"(Closure)强调它"封闭"并捕获了外部环境
   - "Lambda"来自于Lambda演算，是一种数学抽象概念

3. **语言实现**：
   - 在Rust中，这两个术语通常可互换使用
   - 某些语言(如JavaScript)区分匿名函数和闭包
   - 其他语言(如Python)使用lambda关键字创建匿名函数

Rust中所有的闭包都是Lambda函数，但它特别强调了捕获环境的能力，因此更倾向于使用"闭包"这个术语。

总结：`move` 关键字改变了闭包如何与其环境交互，让闭包获取变量的所有权而不是借用它。这在线程编程、异步代码和管理资源生命周期等场景中尤为重要。
*/




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_loop(v.clone());

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }

    #[test]
    fn test_vec_map() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_map(&v);

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }
}
