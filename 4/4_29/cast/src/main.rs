// 型間のキャスト
use std::mem;

fn main() {
    // asでやるキャストは安全なものになる
    let x: i32 = 5;
    let y = x as i64;

    // 有効なキャスト
    // 数値から別の数値型へのキャスト
    // enumから正数へのキャスト
    // boolかcharから正数型へのキャスト
    // u8からcharへのキャスト

    let a = 300 as *const char; //300番地へのポインタ
    let b = a as u32;

    // let a = [0u8, 0u8, 0u8, 0u8];

    // 4つのu8でu32にメモリ上ではなっている
    // let b = a as u32; //asを使った場合エラー
    unsafe {
        // trunsmuteはめっちゃ危険!
        let a = [0u8, 0u8, 0u8, 0u8];
        let b = mem::transmute::<[u8; 4], u32>(a); //u8,4つからu32へのキャスト
    }

    println!("Hello, world!");
}
