// マクロ
// マクロはきれいなコードにするための最終的な手段だそうだ

fn _macro_test() {
    // vec!マクロ
    let _x: Vec<u32> = vec![1, 2, 3];
    // ↓下の短縮形
    let _x: Vec<u32> = {
        let mut temp_vec = Vec::new();
        temp_vec.push(1);
        temp_vec.push(2);
        temp_vec.push(3);
        temp_vec
    };

    // 上のように短縮形をマクロを用いることで作成することができる
    // !(エクスクラメーションマーク)と共に記述する
    macro_rules! _vec {
    // $($name:expr),*で繰り返しを表してる感じ、行列に似てるかも
    ($($x: expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
}

macro_rules! foo {
    (x => $e: expr) => {
        println!("mode X: {}", $e)
    };
    (y => $e:expr) => {
        println!("mode Y: {}", $e)
    };
}

macro_rules! five_times {
    ($x: expr) => {
        5 * $x;
    };
}

// マクロはしっかりと別々のコンテキストで行われる
macro_rules! log {
    ($msg: expr) => {{
        let state: i32 = 5;
        if state > 0 {
            println!("log({}): {}", state, $msg);
        }
    }};
}

macro_rules! write {
    ($w: expr) => {
        println!("$w: {}", $w);
    };
}

// 再帰的マクロ
// 他のマクロを使ったり自身を呼びだしたりできる
macro_rules! write_html {
    ($w:expr, ) => {
        ()
    };
    ($w:expr, $e: tt) => {
        write!($w);
    };
    ($w: expr, $tag: ident [$($inner:tt)* ] $($rest: tt)*) => {{
        write!($w);
        write_html!($w, $($inner)*);
        write!($w);
        write_html!($w, $($rest)*);
    }};
}

mod hyahaaaaa {
    // modの中でmacro_export等を使うとマクロのエクスポートができるっぽい
    #[macro_export]
    macro_rules! hey {
        () => {
            println!("go!");
        };
    }
    // ちなみにmodの前でmacro_useを使うと親のモジュールでもマクロが使えるようになるようだ
}

fn main() {
    println!("Hello, world!");
    foo!(y => 3);

    assert_eq!(25, five_times!(2 + 3));

    let state: &str = "reticulating splines";
    log!(state);

    let mut out = String::new();

    write_html!(&mut out, html[
        head[title["Marcros guide"]]
        body[hl["Macros are the best!"]]
    ]);

    // unreadchable!: 絶対に通らないと予想されている場所で使ってpanicを起こす
    let x: Option<i32> = None;

    match x {
        Some(_) => unreachable!(),
        None => println!("I know x is None"),
    }
}
