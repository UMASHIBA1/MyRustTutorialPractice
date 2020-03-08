use std::result;

fn main() {
    type Name = String;

    let name: Name = "Hello".to_string();

    type Num = i32;
    let x: i32 = 3;
    let y: Num = 3;

    // xとyはi32型とNum型だけど同じものとして認識される。もし違うものとして認識させるようにしたければタプル構造体を使う
    if x == y {
        println!("x == y: true");
    }

    println!("Hello, world!");

    enum ConcreteError {
        Foo,
        Bar,
    }

    // typeエイリアスをジェネリクスとともに使うことも可能
    type Result<T> = result::Result<T, ConcreteError>;
}
