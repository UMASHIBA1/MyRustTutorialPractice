// unsafe
// unsafeを使うことで普通のコードよりも制約が少なくなる

// 関数がアンセーフ
unsafe fn danger_will_robinson() {
    // 恐ろしいもの
}

// アンセーフトレイト
unsafe trait Scary {}

// アンセーフトレイトを実装する
unsafe impl Scary for i32 {}

fn main() {
    // ブロックがアンセーフ
    unsafe {
        // 恐ろしや
    }

    println!("Hello, world!");
}

// unsafeでできること
// 1. static mut で宣言されたミュータブルでグローバルな変数へのアクセスとアップデート
// 2. 生ポインタの参照外し
// アンセーフ関数の呼び出し
