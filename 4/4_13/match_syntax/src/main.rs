fn main() {
    let x = 3;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        5 => println!("five"),
        6 => println!("six"),
        _ => println!("other"),
    };

    // matchは式だから代入できる
    let number = match x {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        // xの可能性全てをコンパイラで確認し、ない時は_を入れる
        _ => "other",
    };
    println!("number: {}", number);

    // 列挙型を扱う時にmatchは便利
    enum Message {
        Quit,
        ChangeColor(i32, i32, i32),
        Move { x: i32, y: i32 },
        Write(String),
    };

    fn quit() {
        println!("Quit!");
    };

    fn change_color(r: i32, g: i32, b: i32) {
        println!("r: {}, g: {}, b{}", r, g, b);
    };

    fn move_to(x: i32, y: i32) {
        println!("Move to ({},{})", x, y);
    };

    let message = Message::Move { x: 1, y: 2 };

    match message {
        Message::Quit => quit(),
        Message::ChangeColor(r, g, b) => change_color(r, g, b),
        Message::Move { x: x, y: y } => move_to(x, y),
        Message::Write(this_message) => println!("message: {}", this_message),
    }

    println!("Hello, world!");
}
