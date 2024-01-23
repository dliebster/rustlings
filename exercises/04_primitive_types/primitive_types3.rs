// primitive_types3.rs
//
// Create an array with at least 100 elements in it where the ??? is.
//
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand
// for a hint.

<<<<<<< HEAD
fn main() {
    let a = [1; 300];
=======

fn main() {
    let a = [1; 200];
>>>>>>> 736502b27024db5ae1b31645f56d91e0bee1b840


    println!("Length: {}", a.len());
    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed")
    }
}
