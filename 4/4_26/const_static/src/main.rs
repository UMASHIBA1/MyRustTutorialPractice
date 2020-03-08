// 定数はプログラム全体のライフタイムで生きている
// コンパイル時にインライン化されるためメモリ上のアドレスは同じとは限らない
const N: i32 = 5;

// インライン化は行われない、メモリ上のアドレスは同じ、プログラム全体のライフタイムを持つ
// mutを使うとミュータブルにできる。unsafeブロック内でやる必要がある
static N2: i32 = 5;

fn main() {
    println!("Hello, world!");
    println!("N: {}", N);
    println!("N2: {}", N2);
}

// constを使う方がおすすめ
