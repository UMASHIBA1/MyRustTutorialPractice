fn main() {
    // &strの方の文字列型について: 固定長でアロケートされるため追加などができない

    // 静的にアロケートされる
    let greeting: &str = "Hello there.";
    println!("greeting: {}", greeting);

    // Rustでは文字列を複数行にわたって書くことができる。
    // stringの場合fooとbarの間の空白も文字列としてカウントされる
    let string = "foo
    bar";

    // \を行の最後に入れることで空白と改行を削れる
    let string2 = "foo\
                   bar";
    assert_eq!("foo\n    bar", string);
    assert_eq!("foobar", string2);

    // Stringの方の文字列型について: 文字列の追加などができる
    let mut string3 = "Hello".to_string(); // string3: String;
    println!("string3: {}", string3);
    string3.push_str(" world");
    println!("pushed string3: {}", string3);

    // 文字列はstring[0]みたいにはインデクシングできない
    let hachiko = "忠犬ハチ公";
    // やるとしたら↓
    let _dog = hachiko.chars().nth(1);

    // 文字列のスライシングはできる、ただしスライシングはバイト単位だから文字に対して区切りが悪いとスライシングできない
    let hachi = &hachiko[0..6];
    println!("hachi: {}", hachi);
}

// &strのString化は必要なければやらない(&strからStringにするのにはメモリに再配置する必要があるから)
