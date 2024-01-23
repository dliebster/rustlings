// structs1.rs
//
// Address all the TODOs to make the tests pass!
//
// Execute `rustlings hint structs1` or use the `hint` watch subcommand for a
// hint.

<<<<<<< HEAD
struct ColorClassicStruct {
    blue: u32,
    red: u32,
    green: u32,
=======

struct ColorClassicStruct {
    red: i32,
    green: i32,
    blue: i32,
>>>>>>> 736502b27024db5ae1b31645f56d91e0bee1b840
}

struct ColorTupleStruct(u32, u32, u32);

#[derive(Debug)]
struct UnitLikeStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // TODO: Instantiate a classic c struct!
        let green = ColorClassicStruct {
            red: 0,
            green: 255,
            blue: 0,
        };

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
<<<<<<< HEAD
        let green = ColorTupleStruct(0, 255, 0);
=======
        // TODO: Instantiate a tuple struct!
        let green = ColorTupleStruct (0, 255, 0);
>>>>>>> 736502b27024db5ae1b31645f56d91e0bee1b840

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
<<<<<<< HEAD
=======
        // TODO: Instantiate a unit-like struct!
>>>>>>> 736502b27024db5ae1b31645f56d91e0bee1b840
        let unit_like_struct = UnitLikeStruct;
        let message = format!("{:?}s are fun!", unit_like_struct);

        assert_eq!(message, "UnitLikeStructs are fun!");
    }
}
