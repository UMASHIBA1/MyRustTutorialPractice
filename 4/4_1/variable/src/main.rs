// 束縛変数

fn main() {
    let x = 5;
    // x = 6; 再代入は不可
    let x: i32 = 2; //再定義は可能(デフォルトでイミュータブル)
    let (x,y) = (1,2);
    let y = "aaa";
    let mut z = 5; //mutを付ける事で変数をミュータブルにできる
    println!("{}", z);
    z = 1;
    println!("{}", z);
    println!("{}",x);
    println!("{}",y);

    println!("Hello, world!");
}
