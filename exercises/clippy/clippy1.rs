// clippy1.rs
// The Clippy tool is a collection of lints to analyze your code
// so you can catch common mistakes and improve your Rust code.
//
// For these exercises the code will fail to compile when there are clippy warnings
// check clippy's suggestions from the output to solve the exercise.
// Execute `rustlings hint clippy1` for hints :)

fn main() {
    let error = f64::EPSILON;
    let x = 0.33f64;
    let y = 0.334f64;

    println!("{} - {}!", (y - x).abs(), error);
    if (y - x).abs() < error {
        println!("Equal!");
    } else {
        println!("Not Equal!");
    }
}
