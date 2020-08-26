// quiz4.rs
// This quiz covers the sections:
// - Modules
// - Macros

// Write a macro that passes the quiz! No hints this time, you can do it!


#[macro_use]
mod macros {

    macro_rules! my_macro {
        // ($val:expr) => { // don't need another block here since format returns immediately
        //     format!("Hello {}", $val);
        // }
        ($val:expr) => {{ // difference was that i needed another block here to act as an expression!!
            let mut x = String::from("Hello ");
            x.push_str($val);
            x
        }}
    }

}

fn main () {
    my_macro!("world!");
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn test_my_macro_world() {
        let x = my_macro!("world!");


        assert_eq!(x, "Hello world!");

    }

    #[test]
    fn test_my_macro_goodbye() {
        assert_eq!(my_macro!("goodbye!"), "Hello goodbye!");
    }
}
