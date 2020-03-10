// 生ポインタ
// ちなみにunsafe
// 有効なメモリを指していることが保証されない、nullでないことも保証されない
// 手動のリソース管理が必要

fn main() {
    println!("Hello, world!");

    // 宣言だけなら大丈夫！
    let x = 5;
    let raw = &x as *const i32;
    let points_at = unsafe { *raw }; //危険！
    println!("raw points at {}", points_at);

    let mut y = 10;
    let raw_mut = &mut y as *mut i32;

    let points_at2 = unsafe { *raw_mut };

    println!("raw mut points at {}", points_at2);

    // 明示的なキャスト
    let i: u32 = 1;
    let p_imm: *const u32 = &i as *const u32;

    // 暗示的なキャスト
    let mut m: u32 = 2;
    let p_mut: *mut u32 = &mut m;

    unsafe {
        let ref_imm: &u32 = &*p_imm;
        let ref_mut: &mut u32 = &mut *p_mut;
    }
}
