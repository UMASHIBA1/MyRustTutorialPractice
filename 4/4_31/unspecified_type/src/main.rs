// サイズ不定型

fn main() {
    println!("Hello, world!");
}

// サイズ不定型はポインタを通してのみ操作可能
// 変数や引数は動的なサイズを持つことができない
// structの最後のフィールドのみ動的なサイズを持てる

trait Foo {
    fn foo() -> ();
}

impl Foo for str {
    fn foo() {
        println!("foo");
    }
}

// この書き方はダメ！
// impl Foo for [T] {
//     fn foo() {
//         println!("foo3");
//     }
// }

// これでOK
impl Foo for &str {
    fn foo() {
        println!("foo2");
    }
}

// もし動的サイズ型を引数にとれるような関数を定義した場合、特別な境界?Sizedを使用可能
struct Foo2<T: ?Sized> {
    f: T,
}
