// メモ: ヒープはCでいうところのmallocのようなもの

fn ownership_test() {
    let v = vec![1, 2, 3]; //変数束縛vがvec![1,2,3]の所有権を持っている
    let v2 = v; //vからv2にvec![1,2,3]の所有権が転送した。
    println!("v[0] is: {}", v[0]); //Error! vはこの時点で所有権をv2に転送したからvからvec![1,2,3]を参照できない
    println!("v2[1]: {}", v2[1]);
}

fn double(x: i32) {
    x * 2
}

fn copy_trait() {
    // primitive型等はCopyトレイトがあるため所有権ルールに従わない。トレイトはメソッドのようなものだと現時点で考えている
    let v = 5;
    let v2 = a;
    println!("a is {}", v);

    let x = 4;
    let _y = double(4);
    println!("x is {}", x);
}

fn main() {
    println!("Hello, world!");
    ownership_test();
}
