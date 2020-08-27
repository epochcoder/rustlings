// option2.rs
// Make me compile! Execute `rustlings hint option2` for hints

fn main() {
    let optional_value = Some(String::from("rustlings"));
    // TODO: Make this an if let statement whose value is "Some" type
    if let Some(value) = optional_value {
        println!("the value of optional value is: {}", value);
    } else {
        println!("The optional value doesn't contain anything!");
    }

    let mut optional_values_vec: Vec<Option<i8>> = Vec::new();
    // optional_values_vec.push(None);
    for x in 1..10 {
        optional_values_vec.push(Some(x));
    }
    //optional_values_vec.push(None);

    // TODO: make this a while let statement - remember that vector.pop also adds another layer of Option<T>
    // You can stack `Option<T>`'s into while let and if let
   // println!("first pop: {:?}", optional_values_vec.pop());

    //  there is some real weirdness if you stack optionals in a while let, the first none occurrence stops the while
    // which makes sense actually,
    // not a good way for iterating a vector...

    while let Some(Some(number)) = optional_values_vec.pop() {
        println!("Number: {}", number);
    }

    // for opt in &vector {
    //     if opt.is_some() {
    //         println!("Number: {}", opt.unwrap());
    //     }
    // }

    // let mut iter = vector.iter();
    // while let Some(iterNext) = iter.next() {
    //     if let Some(num) = iterNext {
    //         println!("Number: {}", num);
    //     }
    // }

    // let mut count = 0;
    // while let Some(None) = optional_values_vec.pop() {
    //     println!("empty value {}", count = count + 1);
    // }
    //
    // while let Some(value) = optional_values_vec.pop() {
    //     println!("current value: {:?}", value);
    //
    //     if let Some(val) = value {
    //         println!("\tfound val: {}", val);
    //     } else {
    //         println!("\tno val found");
    //     }
    // }
    //
    // println!("---------");
    //
    // for x in optional_values_vec {
    //     if let Some(val) = x {
    //         println!("found val: {}", val);
    //     } else {
    //         println!("no val found");
    //     }
    // }
}
