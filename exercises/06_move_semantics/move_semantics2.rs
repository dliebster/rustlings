// move_semantics2.rs
//
// Make the test pass by finding a way to keep both Vecs separate!
//
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand
// for a hint.

<<<<<<< HEAD
=======

>>>>>>> 736502b27024db5ae1b31645f56d91e0bee1b840
#[test]
fn main() {
    let vec0 = vec![22, 44, 66];

<<<<<<< HEAD
    let mut vec1 = fill_vec(&vec0);
=======
    let mut vec1 = fill_vec(vec0.clone());
>>>>>>> 736502b27024db5ae1b31645f56d91e0bee1b840

    assert_eq!(vec0, vec![22, 44, 66]);
    assert_eq!(vec1, vec![22, 44, 66, 88]);
}

<<<<<<< HEAD
fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
    let mut vec = vec.clone();
=======
fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    let mut new_vec = vec![];
    for i in vec.iter_mut() {
        new_vec.push(*i);
    }
>>>>>>> 736502b27024db5ae1b31645f56d91e0bee1b840

    new_vec.push(88);

    new_vec
}
