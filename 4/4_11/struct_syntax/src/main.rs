// structが持つ値はイミュータブルがデフォルト(他と同じ)
struct Point {
    x: i32,
    y: i32,
}

struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

// タプル構造体
struct Color(i32, i32, i32);

fn main() {
    println!("Hello, world!");
    let origin = Point { x: 0, y: 0 };
    println!("x: {}, y: {}", origin.x, origin.y);
    // mutでミュータブルにできる
    let mut mutable_origin = Point { x: 0, y: 0 };
    mutable_origin.x = 1;
    println!(
        "mutable origin, x: {}, y: {}",
        mutable_origin.x, mutable_origin.y
    );
    // 一時的にミュータブルにできる
    let mut point = Point { x: 0, y: 0 };
    point.x = 1;
    let point = point; //これでイミュータブルになる
                       // point.y = 1; // Error!

    // 下のようにプロパティの一部を簡単にコピーして使うことができる
    let mut point = Point3D { x: 0, y: 0, z: 0 };
    point = Point3D { y: 1, ..point };

    // タプル構造体の使い方
    let black = Color(0, 0, 0);
    // ほとんどの場合、structを使った方がいいらしいよ

    // Unit-like構造体
    struct Electron;
    let x = Electron;
}

// まとめ
// structがあるよ
// structは宣言するときにプロパティの一部をコピーして簡単に宣言できる
// タプル構造体があるよ
// Unit-like構造体があるよ
