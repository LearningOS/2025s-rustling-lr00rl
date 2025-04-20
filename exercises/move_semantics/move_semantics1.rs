// move_semantics1.rs
//
// Execute `rustlings hint move_semantics1` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0); // vec0 sent to fill_vec func, and borrowed value to vec1

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);  // here used vec1
    // if you use {} for vec1, print:
/*
⚠️  Compilation of exercises/move_semantics/move_semantics1.rs failed!, Compiler error message:

error[E0277]: `Vec<i32>` doesn't implement `std::fmt::Display`
  --> exercises/move_semantics/move_semantics1.rs:12:67
   |
12 |     println!("{} has length {} content `{}`", "vec1", vec1.len(), vec1);
   |                                                                   ^^^^ `Vec<i32>` cannot be formatted with the default formatter
   |
   = help: the trait `std::fmt::Display` is not implemented for `Vec<i32>`
   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0277`.
*/

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
