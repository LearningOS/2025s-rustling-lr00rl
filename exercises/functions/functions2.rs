// functions2.rs
//
// Execute `rustlings hint functions2` or use the `hint` watch subcommand for a
// hint.


fn main() {
    call_me(3);
    call_me(-3);
}

fn call_me(num: i32) {
    println!("Call the function! <- got {}", num);
    // here is i32, with signed it
    // and here, you can use ..num to call a range, just like for i in range(start, end, step)
    println!("print with step 1");
    ////////////////////////////////////////
    // The valid range syntaxes in Rust are:
    // start..end (Exclusive end, e.g., 0..10 gives 0 through 9)
    // start..=end (Inclusive end, e.g., 0..=10 gives 0 through 10)
    // start.. (From start onwards)
    // ..end (From the beginning up to, but not including, end)
    // ..=end (From the beginning up to and including end)
    // .. (Full range)
    //
    // If start < end: The range includes values from start up to end-1.
    // 0..5 iterates 0, 1, 2, 3, 4.
    // If start == end: The range is empty. It's valid, but the iterator will immediately signal it's finished. A for loop over this range will not execute its body.
    // 5..5 iterates over nothing.
    // If start > end: The range is empty. Similar to the case above, it's valid but represents an empty sequence.
    // 10..5 iterates over nothing.
    ////////////////////////////////////////
    // num = num.abs();  // if you want to do this, you have to declear the num as mut num: fn call_me(mut num: i32) {
    for i in -1*num..num {
        println!("Ring! Call number {}", i);
    }
    // or with step
    println!("print with step 2");
    for i in (-1*num..num).step_by(2) {
        println!("Ring! Call number {}", i);
    }
    println!("end the calling.\n")
}
