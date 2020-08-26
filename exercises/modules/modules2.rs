// modules2.rs
// Make me compile! Execute `rustlings hint modules2` for hints :)

use delicious_snacks::fruits::PEAR as fruit;
use delicious_snacks::veggies::CUCUMBER as veggie;

mod delicious_snacks {

    pub mod fruits {
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    pub mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        fruit,
        veggie
    );
}
