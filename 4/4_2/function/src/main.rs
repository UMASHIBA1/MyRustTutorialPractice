fn print_x(x: i32) {
    println!("x is: {}", x);
}

// fn print_x_no_type(x) { 引数に型がないとエラー
//     println!("x is: {}", x);
// }

fn print_sum(x: i32, y: i32) {
    println!("{}", x + y);
}

fn add_one(x: i32)->i32 {
    x + 1 // セミコロンがない
    // x + 1; エラーになる
}

// Rustには式(1+2とか3*8とか)と2種類の文がある
// 式は値を返す、文は返さない
// 宣言文
// let x = 5;
// 式文
// 式の最後に;(セミコロン)を付けることで式を文にする

// ------------

fn foo(x: i32) ->i32 {
    return x; // Rustでは処理の途中で返すときのみreturn文を使う

    // x + 1 //最後はセミコロン無しの値(returnなし)
}

// 発散する関数(値を返さない関数)のための構文
// println!("", );
// panic!("this thread crashed!");

fn crash_with_message() {
    panic!("crash with message! yeaaahhhhh");
}


// function pointer(関数ポインター)
// 関数を指す変数束縛を作ることも可能

fn plus_one(i: i32)->i32 {
    i + 1
}


fn main() {
    print_x(3);
    print_sum(3,2);
    println!("add_one{}", add_one(6));
    foo(12);
    let f: fn(i32) -> i32 = plus_one;
    println!("f(plus_one): {}", f(10));
    crash_with_message();
}
