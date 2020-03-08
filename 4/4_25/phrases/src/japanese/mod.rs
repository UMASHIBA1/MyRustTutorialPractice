// 下の階層の関数をこちらで再エクスポートする
// selfで現在位置から指定できる
pub use self::farewells::goodbye;
pub use self::greetings::hello;

mod farewells;
mod greetings;
