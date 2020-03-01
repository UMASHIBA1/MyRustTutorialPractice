// if let
// パターンマッチにうまく当てはめることができればifとletを一緒に行える

fn main() {
    println!("Hello, world!");
    let y = true;
    let option = if y { Some(5) } else { None };
    // ifとletを一緒にしたい時便利だよ
    if let Some(x) = option {
        println!("x: {}", x);
    }

    // while let
    // 値がパターンにマッチし続ける限りループする
    let mut v = vec![1, 3, 5, 7, 11];
    while let Some(x) = v.pop() {
        println!("{}", x);
    }
}
