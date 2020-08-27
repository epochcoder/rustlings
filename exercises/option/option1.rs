// option1.rs
// Make me compile! Execute `rustlings hint option1` for hints

// you can modify anything EXCEPT for this function's sig
fn print_number(maybe_number: Option<u16>) {
    println!("printing: {}", maybe_number.unwrap());
}

fn main() {
    print_number(Some(13));
    print_number(Some(99));

    let mut numbers: [Option<u16>; 33] = [None; 33];
    assign_numbers(&mut numbers);

    // see https://stackoverflow.com/questions/33036859/why-does-println-work-only-for-arrays-with-a-length-less-than-33
    for i in 0..numbers.len() {
        println!("got number {:?}", &numbers[i]);
    }
}

fn assign_numbers(numbers: &mut [Option<u16>]) {
    for iter in 0..5 {
        let number_to_add: u16 = {
            ((iter * 1235) + 2) / (4 * 16)
        };

        numbers[iter as usize] = Some(number_to_add);
    }
}
