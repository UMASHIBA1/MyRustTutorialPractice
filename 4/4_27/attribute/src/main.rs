// #[something]で次のアイテムに適用される
#[test]
struct Foo;

mod bar {
    // #![something]でこれを囲んでるアイテムに適用される
    #![test]
}

// testの時に実行される、通常のコンパイルではコンパイルされない
#[test]
fn check() {
    assert_eq!(2, 1 + 1);
}

#[inline(always)]
fn super_fast_fn() {}

// #[cfg(target_os = "macos")]
// mod macos_only;

fn main() {
    println!("Hello, world!");
}

// アトリビュートはRustコンパイラによって定義されている以外は許可されていない
