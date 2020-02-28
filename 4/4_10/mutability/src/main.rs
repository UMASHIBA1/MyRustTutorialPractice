fn mutability() {
    // デフォルトでイミュータブル
    let x = 5;
    // x = 6; //Error!
    let mut y = 1;
    y = 2;

    let mut xx = 5;
    let mut yy = &mut xx;
}

fn in_and_out_mutability() {
    use std::sync::Arc;
    let x = Arc::new(5);
    let y = x.clone();
    // メソッドを介して渡す事はちょっとできるっぽい。もちろん二つ以上mutの借用をすると死ぬ
}

// フィールドレベルのミュータビリティ
// struct Point {
//     x: i32,
//     mut y: i32, //ダメ: 一部がミュータブルで一部がイミュータブルなstructは作れない
// }

struct Point {
    x: i32,
    y: i32,
}

use std::cell::Cell;
struct Point2 {
    x: i32,
    y: Cell<i32>,
}

fn main() {
    println!("Hello, world!");
    let mut a = Point { x: 5, y: 6 };
    a.x = 10;

    let b = Point { x: 5, y: 6 };
    // b.x = 10;  // Error!

    // ただしCellを使えばフィールドレベルでも値を変更できる
    let point = Point2 {
        x: 5,
        y: Cell::new((6)),
    };
    point.y.set(7);
    println!("y: {:?}", point.y);
}

// つまり何が言いたいかっていうとCellとかのインスタンスを使えば変更可能だよってことかな
