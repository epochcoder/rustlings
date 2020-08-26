// strings1.rs
// Make me compile without changing the function signature!
// Execute `rustlings hint strings1` for hints ;)

fn main() {
    let mut answer = &mut current_favorite_color();
    {
        let mut answer2 = &answer;
    }

    println!("My current favorite color is {}", answer);

    consume(answer);


    println!("My current favorite color is also {}", answer);

}

fn consume(alababa: &mut String) {
    alababa.push_str("blah");
    println!("adding consume blah {}", alababa);
}

fn current_favorite_color() -> String {
    String::from("blue")
}
