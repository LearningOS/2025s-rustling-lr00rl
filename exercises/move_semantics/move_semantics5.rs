// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let mut x = 100;
    println!("var x: {}", x);
    let y = &mut x;
    println!("var y: {:p}", y);
    println!("var *y: {}", *y);
    *y += 100;
    println!("var y: {}", y);
    println!("var *y: {}", *y);
    // why output y = *y
    /*
    在 Rust 中，当打印可变引用（&mut T）时，println! 宏会自动解引用它以显示它指向的值。这就是为什么 println!("var y: {}", y); 和 println!("var *y: {}", *y); 显示相同的结果。
    尽管打印结果相同，但 *y 和 y 在实际使用中是不同的：

    y 是一个引用（指针）本身
    *y 是引用指向的值

    使用 * 解引用运算符的必要性在于：

    修改值：当你需要通过引用修改值时，必须使用 *y += 100; 而不能直接写 y += 100;（后者会试图修改引用本身，而不是它指向的值）。
    访问方法和字段：如果你需要访问引用指向的值的方法或字段，可以直接使用 y.method() 或 y.field，Rust 会自动解引用。
    类型区别：在更复杂的代码中，区分引用和它指向的值对于类型检查非常重要。
    显式性：使用 *y 可以明确表示你正在操作引用指向的值，而不是引用本身，这使代码更加清晰。

    所以，虽然在打印时结果相同，但解引用运算符 * 在实际操作引用指向的值时是必不可少的。
     */
    let z = &mut x;
    *z += 1000;
    // both y and z are borrow the x
    assert_eq!(x, 1200);
}
