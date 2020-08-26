// enums2.rs
// Make me compile! Execute `rustlings hint enums2` for hints!


#[derive(Debug)]
enum Message {
    Move { x: i32, y: i32 },
    Echo(String),
    ChangeColor(i32, i32, i32),
    Quit
}

impl Message {
    fn call(&self) {
        println!("{:?}", &self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();

        match message {
            Message::Move{ x, y } => {
                move_house_to((x, y))
            }
            _ => println!("Not doing any moving today")
        }
    }
}

fn move_house_to(location: (&i32, &i32)) {
    println!("moving the house to x:{} y:{}", location.0, location.1);
}
