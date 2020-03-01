// トレイト

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

// ある型が提供しなければならない機能をRustコンパイラに伝える言語機能
// traitは型を定義する物
// つまりそのメソッドを持っているstructを指定する型ということ
trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

struct Square {
    x: f64,
    y: f64,
    side: f64,
}

impl HasArea for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

// ジェネリック関数におけるトレイト境界

// あるメソッドが存在している型のみを受け取りたい場合などに便利
// この場合print_area関数の引数はareaメソッドを持つことが保証される
fn print_area<T: HasArea>(shape: T) {
    println!("This shape has an area of {}", shape.area());
}

// ジェネリック構造体におけるトレイト境界

struct Rectangle<T> {
    x: T,
    y: T,
    width: T,
    height: T,
}

// Tの型にeqという==で比較することが可能なメソッドがあるかということ
impl<T: PartialEq> Rectangle<T> {
    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

// 複数のトレイト境界
use std::fmt::Debug;

fn multi_trait<T: Clone + Debug>(x: T) {
    x.clone();
    println!("{:?}", x);
}

// Where節

fn previous_where<T: Clone, K: Clone + Debug>(x: T, y: K) {
    x.clone();
    y.clone();
    println!("{:?}", y);
}

// 見やすくなったﾖ！
fn where_trait_test1<T, K>(x: T, y: K)
where
    T: Clone,
    K: Clone + Debug,
{
    x.clone();
    y.clone();
    println!("{:?}", y);
}

// デフォルトメソッド
// 実装者がやりそうなメソッドが分かっているのならばtraitの時点でデフォルトメソッドとして実装できる
trait ValidMethod {
    fn is_valid(&self) -> bool;
    fn is_invalid(&self) -> bool {
        !self.is_valid()
    }
}

// トレイトの継承
trait Foo {
    fn foo(&self);
}

trait FooBar: Foo {
    fn foobar(&self);
}

struct Baz;

// 継承した場合はどちらも実装する必要がある。
impl Foo for Baz {
    fn foo(&self) {
        println!("foo");
    }
}
impl FooBar for Baz {
    fn foobar(&self) {
        println!("foobar");
    }
}

fn main() {
    let circle1 = Circle {
        x: 0.0f64,
        y: 0.0f64,
        radius: 1.0f64,
    };

    let square1 = Square {
        x: 0.0f64,
        y: 0.0f64,
        side: 1.0f64,
    };

    print_area(circle1);
    print_area(square1);

    let mut rec = Rectangle {
        x: 0,
        y: 0,
        width: 47,
        height: 47,
    };

    assert!(rec.is_square());
    rec.height = 42;
    assert!(!rec.is_square());

    // トレイトの制限について
    // 1. 別のスコープで宣言されたトレイトはこちらのスコープでは適用されない(useを使う必要がある)
    // 2. implとtraitは同じ人によって実装されなければいけない?(ちょっとまだ解釈できてない)

    // fileのパスの指定がうまくいかなくて動かないからコメント化
    // use std::io::Write;

    // let mut file1 = std::fs::File::open("../../../test.txt").expect("Coundn't open test.txt");
    // let buf = b"hello world";
    // let result = file1.write(buf);

    // println!("Hello, world!");

    multi_trait(5);
}
