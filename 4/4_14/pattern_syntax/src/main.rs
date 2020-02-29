// パターンについて

fn main() {
    let x = "X";
    let c = "c";

    match c {
        // match文の中でシャドーイングが行われる。
        x => println!("x: {}, c: {}", x, c),
    }
    println!("x: {}", x);

    // 複式パターン
    let x = 2;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("other"),
    }

    // 分配束縛
    struct Point {
        x: i32,
        y: i32,
    }

    let origin = Point { x: 0, y: 0 };

    // 便利に束縛できる
    match origin {
        Point { x, y } => println!("({},{})", x, y),
    }

    // 一部だけ取得
    match origin {
        Point {x, ..} => println!("x is {}", x);
    }

    // 別名を付ける
    match origin {
        Point {x: x1, y: y1} => println!("x1 is {}, y1 is {}",x1, y1 );
    }

    // 参照を取得
    let x = 5;

    match x {
        ref r => println!("Got a reference to {}", r); //r = &i32
    }

    match x {
        ref mut r => println!("Got a reference to {}", r); //r = &i32(ミュータブル)
    }

    // 範囲
    match x {
        1 ..5 => println!("one through five"),
        _ => println!("anything"),
    }

    let x = 😁;

    // 文字列でも範囲で指定が可能
    match x {
        e @ 'a'...'j' => println!("e: {}", e), // @で取得した値を束縛可能(eに束縛)
        'k'...'z' => println!("k to z"),
        _ => println!("something else"),
    }

    // マッチガード
    let x = 4;
    let y = false;

    match x {
        4 | 5 if y => println!("yes"), // if文を使うことで実行するか指定できる
        _ => println!("no");
    }

    println!("Hello, world!");
}


// 本当にたくさんのマッチ方法がある。ドキュメントを参照しながら色々と試して使いこなせるようになりたい。
