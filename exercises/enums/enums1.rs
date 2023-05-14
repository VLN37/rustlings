// enums1.rs
// No hints this time! ;)

// derive prints the str representation in this case
#[derive(Debug)]
enum Message {
    Quit,
    Echo,
    Move,
    ChangeColor
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
