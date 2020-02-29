// Rustにはenum型というものがある。
// 自分なりの解釈ではTypeScriptでいうところの type Message = Quit | ChangeColor | Move | Write;みたいな感じ
enum Message {
    Quit,                       //データをも持たないヴァリアントの定義方法
    ChangeColor(i32, i32, i32), // タプル構造体のような名前なしの構造体の定義方法
    Move { x: i32, y: i32 },    // 名前あり構造体の定義方法
    Write(String),              //データを一つもつ定義方法
}

fn enum_test() {
    let message: Message = Message::Move { x: 32, y: 24 };
}

fn main() {
    println!("Hello, world!");
}
