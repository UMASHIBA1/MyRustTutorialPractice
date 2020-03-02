// トレイトオブジェクト

trait Foo {
    fn method(&self) -> String;
}

impl Foo for u8 {
    fn method(&self) -> String {
        format!("u8: {}", *self)
    }
}

impl Foo for String {
    fn method(&self) -> String {
        format!("string: {}", *self)
    }
}

// 静的ディスパッチ
// このような関数にすることでコンパイル時にインライン化(関数でなく一連のコードとしてコンパイルすること)し効率が良くなる
// 基本は静的ディスパッチを使った方が良いらしい
fn do_something<T: Foo>(x: T) {
    x.method();
}
// ↓この指定方法は以下のようにそれぞれの型用の関数にコンパイラが解釈してくれる

// fn do_something_u8(x: u8) {
//     x.method();
// }

// fn do_something_string(x: String) {
//     x.method();
// }

// 動的ディスパッチ
// 動的ディスパッチは実行時に初めて型が判明する
// これによってコードの膨張は抑えられる
fn do_something2(x: &Foo) {
    x.method();
}

// トレイトオブジェクトの内部表現
// トレイトオブジェクトはvtableというメソッドへのポインターのリストが入っているレコードをコンパイラが作成している

// トレイトオブジェクトにできるのはオブジェクト安全なトレイトのみ
// 条件,
// ・トレイトがSelf: Sizedを要求しない
// ・トレイトのメソッド全てがオブジェクト安全であること
// ではメソッドのオブジェクト安全とは？
// ・どのような型パラメータも持ってはならない
// ・Selfを使ってはならない

fn main() {
    // 普通
    let x = 5u8;
    let y = "Hello".to_string();

    // 静的ディスパッチ
    do_something(x);
    do_something(y);

    // 動的ディスパッチ
    // この場合&xがトレイトオブジェクト(トレイトオブジェクトはas &Fooのようにキャストすることで得られる)
    do_something2(&x as &Foo);
}
