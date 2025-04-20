// functions3.rs
//
// Execute `rustlings hint functions3` or use the `hint` watch subcommand for a
// hint.


fn main() {
    call_me(3);
    // if you call the call_me(-3); you will got
    // error[E0600]: cannot apply unary operator `-` to type `u32`
    //  --> exercises/functions/functions3.rs:9:13
    //   |
    // 9 |     call_me(-3);
    //   |             ^^ cannot apply unary operator `-`
    //   |
    //   = note: unsigned values cannot be negated
    // 
    // error: aborting due to 1 previous error
    // 
    // For more information about this error, try `rustc --explain E0600`.

}

fn call_me(num: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
