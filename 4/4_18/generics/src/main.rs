// ジェネリック: 型に自由度を与える

// ジェネリックの最初のパラメータは慣習からTにする
enum GenericTest<T, B> {
    Good(T),
    Bad(B),
}

fn generics_test() {
    let x: Option<i32> = Option::Some(5);
    let generic_test1: GenericTest<i32, f64> = if x.unwrap() > 5 {
        GenericTest::Good(5)
    } else {
        GenericTest::Bad(2.0)
    };
}

// ジェネリック関数
fn double<T>(x: T) {
    // xに関する処理何か
}

// ジェネリック構造体
struct Point<T> {
    x: T,
    y: T,
}

// ジェネリック構造体へのメソッドの追加方法
impl<T> Point<T> {
    fn swap(&mut self) {
        std::mem::swap(&mut self.x, &mut self.y);
    }
}

fn main() {
    let int_origin = Point { x: 0, y: 0 };

    generics_test();
}
