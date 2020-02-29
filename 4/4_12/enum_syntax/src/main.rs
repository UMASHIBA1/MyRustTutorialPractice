// Rustにはenum型というものがある。
// 自分なりの解釈ではTypeScriptでいうところの type Message = Quit | ChangeColor | Move | Write;みたいな感じ
enum Message {
    Quit,
    ChangeColor(i32, i32, i32),
    Move { x: i32, y: i32 },
    Write(String),
}

fn enum_test() {
    let message: Message = Message::Move { x: 32, y: 24 };
}

fn main() {
    println!("Hello, world!");
}
