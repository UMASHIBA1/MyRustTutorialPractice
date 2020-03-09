// Derefによる型強制
use std::ops::Deref;

struct DerefExample<T> {
    value: T,
}

// *(ポインタ演算子)をオーバーロードする!
impl<T> Deref for DerefExample<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.value
    }
}

// derefによる型強制

fn foo(_s: &str) {
    // 一瞬だけ文字列を借用する
}

fn foo2(_s: &[i32]) {
    // スライスを借用
}

fn main() {
    let x = DerefExample { value: 'a' };
    assert_eq!('a', *x);

    // String型はDerefを実装している為fooを使う時に&strに型強制できる
    let owned = "Hello".to_string();

    foo(&owned);

    let owned2 = vec![1, 2, 3];

    foo2(&owned2);

    struct Foo;
    impl Foo {
        fn foo(&self) {
            println!("Foo");
        }
    }

    let f = &&Foo;
    // 以下は全て同じ
    f.foo();
    (&f).foo();
    (&&f).foo();
    (&&&&&&&&&&f).foo();
}
