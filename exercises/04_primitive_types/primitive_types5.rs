 // primitive_types5.rs
//
// Destructure the `cat` tuple so that the println will work.
//
// Execute `rustlings hint primitive_types5` or use the `hint` watch subcommand
// for a hint.

<<<<<<< HEAD
fn main() {
    let cat = ("Furry McFurson", 3.5);
    let name = cat.0;
    let age = cat.1;
=======

fn main() {
    let cat = ("Furry McFurson", 3.5);
    let (name, age) = cat;
>>>>>>> 736502b27024db5ae1b31645f56d91e0bee1b840

    println!("{} is {} years old.", name, age);
}
