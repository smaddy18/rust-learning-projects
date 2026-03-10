enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn process(msg: Message) {
    match msg {
        Message::Quit => println!("I'm quitting..."),
        Message::Move { x, y } => println!("Moving to ({},{})", x, y),
        Message::Write(text) => println!("Message: {}", text),
        Message::ChangeColor(R, G, B) => println!("The new color in RGB : ({}, {}, {})", R, G, B),
    }
}

fn main() {
    let quit = Message::Quit;
    let msg_move = Message::Move { x: 3, y: 5 };
    let write = Message::Write("Hello".to_string());
    let color = Message::ChangeColor(115, 255, 100);

    process(quit);
    process(msg_move);
    process(write);
    process(color);
}
