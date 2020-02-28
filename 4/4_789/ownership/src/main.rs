// 4-7: 所有権

// メモ: ヒープはCでいうところのmallocのようなもの

fn ownership_test() {
    let v = vec![1, 2, 3]; //変数束縛vがvec![1,2,3]の所有権を持っている
    let v2 = v; //vからv2にvec![1,2,3]の所有権が転送した。
                // println!("v[0] is: {}", v[0]); //Error! vはこの時点で所有権をv2に転送したからvからvec![1,2,3]を参照できない
    println!("v2[1]: {}", v2[1]);
}

fn double(x: i32) -> i32 {
    x * 2
}

fn copy_trait() {
    // primitive型等はCopyトレイトがあるため所有権ルールに従わない。トレイトはメソッドのようなものだと現時点で考えている
    let v = 5;
    let _v2 = v;
    println!("a is {}", v);

    let x = 4;
    let _y = double(4);
    println!("x is {}", x);
}

// 4-8: 参照と借用

// 参照を渡すことで所有権を借用するという意味になる、これによってブロックが終了した後も親のブロックの方で参照先の値が使える
// ただし、この参照方法は値を変えることはできない
fn borrow_ownership(_v1: &Vec<i32>, _v2: &Vec<i32>) -> i32 {
    // v1,v2に関する操作
    42
}

fn mut_borrow() {
    let mut x = 5;
    {
        // mut参照というもの、普通の参照との違いはアクセスする際に*をつける
        // この参照方法は値を変えることができる、ただし複数のmut参照が存在することはない
        let y = &mut x;
        *y += 1;
    }
    println!("{}", x);
    // println!("{}", y); Error!: 借用は所有者のスコープより長く存在していてはいけない
}

fn main() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![1, 2, 3];
    println!("Hello, world!");
    ownership_test();
    borrow_ownership(&v1, &v2);
    mut_borrow();
}

// つまりRustの参照で何もないところを指すのは許されない

// 4-9: ライフタイム
// ライフタイムは返り値などで返した時にそれのスコープがどこかを判別するための識別子

fn lifetime<'a>(x: &'a i32) {
    struct Foo<'a> {
        x: &'a i32,
    }
    impl<'a> Foo<'a> {
        fn x(&self) -> &'a i32 {
            self.x
        }
    }
}
