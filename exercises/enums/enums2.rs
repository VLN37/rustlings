// enums2.rs
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a hint.

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

struct PointTuple(u8, u8);

impl Point {
    fn build(x: i32, y: i32) -> Point {
        Point {x, y}
    }
}

struct Color {
    red: i8,
    green: i8,
    blue: i8
}

struct ColorTuple(i32, i32, i32);

#[derive(Debug)]
enum Message {
    // TODO: define the different variants used below
    Quit,
    Echo(String),
    Move{x: i32, y: i32},
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let messages = [
        Message::Move {x: 10, y: 30},
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(255, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
