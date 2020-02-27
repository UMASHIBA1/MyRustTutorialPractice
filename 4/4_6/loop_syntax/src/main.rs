fn loop_syntax() {
    // let mut count = 0;
    // 無限ループを作りたい時はwhileではなくloopを使う
    // loop {
    //     println!("loop!");
    // }

    let mut count2 = 1;
    let mut done = false;

    while !done {
        count2 += count2;
        println!("while count: {}", count2);
        if count2 > 5 {
            done = true;
        }
    }

    // Rustのfor文, 何回回すか決まってるときに使う
    for x in 0..10 {
        println!("for syntax x is {}", x);
    }

    // enumerateで何回目のループかを知ることができる
    for (i, j) in (5..10).enumerate() {
        println!("i = {}, j = {}", i, j);
    }

    // breakとかcontinueとかも使えるよ
    let mut x = 5;
    loop {
        x += x;
        println!("x{}", x);
        if x > 30 {
            break;
        };
    }

    for i in 0..10 {
        if i % 2 == 0 {
            continue;
        }
        println!("i: {}", i);
    }

    // # ループラベル!
    'outer: for x in 0..10 {
        'inner: for y in 0..10 {
            if x % 2 == 0 {
                continue 'outer;
            } // xのループを継続
            if y % 2 == 0 {
                continue 'inner;
            } // yのループを継続
            println!("x: {}, y: {}", x, y);
        }
    }
}

fn main() {
    println!("Hello, world!");
    loop_syntax();
}
