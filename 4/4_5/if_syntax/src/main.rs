fn if_syntax() {
    let x = 6;
    // 条件式を()で囲まないところがポイント
    if x == 5 {
        println!("xは５です");
    } else if x==6 {
        println!("xは6です");
    }
     else {
        println!("xは5ではない");
    }

    let y = if x==5{10} else if x==6 {12} else{15};
    println!("yは{}", y);

    // if文はelseがないと結果が必ず()になる
    // let z = if x==5 {12};
}

fn main() {
    println!("Hello, world!");
    if_syntax();
}
