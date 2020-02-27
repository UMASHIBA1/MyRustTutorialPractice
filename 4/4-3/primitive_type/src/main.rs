// Rustに存在しているプリミティブ型について

fn primitive_list() {
    // bool
    let bo = true;
    let bo: bool = true;

    // char(Rustのcharは1バイトでなく4バイト)
    let ch = 'x';
    let ch = '💕';

    // 数値型
    // [i8,i16,i32,i64]: 符号付き8bit~64bit
    // [u16,u32,u64]: 符号なし8bit~64bit
    // isize: そのマシンのポインタの大きさに合わせた符号付きの数値
    // usize: そのマシンのポインタの大きさに合わせた符号なしの数値
    // [f32,f64]: 浮動小数点型, 32bit,64bit
    let num: u8 = 1;
    let num: i16 = -128;
    let f32 = 1.0;

    // 配列(デフォルトでイミュータブル！);
    let list = [1,2,3]; //a: [i32; 3]
    let mut m = [1,2,3]; //m: [i32; 3]
    // 0のみの配列、長さ20
    let a = [0; 20];
    let length = a.len();
    let three_element = a[3];
    // スライス
    let example_list = [0,1,2,3,4];
    let complete = &example_list[..]; //example_listに含まれる全ての要素を持つスライス
    let middle = &example_list[1..4]; // 1,2,3のみを要素に持つスライス

    // str
    // なんかあるらしい、詳しい説明はまだない

    // タプル
    let tupple1: (i32, &str) = (1, "hello");
    let tupple1_0 = tupple1.0; // .0みたいにindexを.で指定することでアクセス可能
    let (x,y,z) = (1,2,3); // こんな感じで

}

fn main() {
    println!("Hello, world!");
}
