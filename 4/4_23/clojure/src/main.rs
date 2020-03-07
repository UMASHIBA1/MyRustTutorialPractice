// クロージャ

fn clojure_test() {
    // クロージャは返り値の型を示さない
    let plus_one = |x: i32| x + 1;

    assert_eq!(2, plus_one(1));

    // 複数行のクロージャを書くことも可能
    let plus_two = |x| {
        let mut result: i32 = x;
        result += 1;
        result += 1;
        result
    };

    assert_eq!(4, plus_two(2));

    // クロージャを囲んでるスコープ内の束縛はクロージャの中で使える
    {
        let num = 5;
        let plus_num = |x: i32| x + num;
        assert_eq!(10, plus_num(5));
    }

    // Error! plus_numで既にnumが借用されておりスコープ外になっていないので返却もされていない、よってyでミュータブルな借用はできない
    // {
    //     let num = 5;
    //     let plus_num = |x: i32| x + num;
    //     let y = &mut num;
    // }

    // moveクロージャ
    // moveキーワードでクロージャに環境の所有権の取得を強制することが可能
    let num = 5;
    // numのCopyの所有権を取得
    let owns_num = move |x: i32| x + num;

    // add_numはミュータブルなnumの参照を取得し変更
    {
        let mut num = 5;
        {
            let mut add_num = |x: i32| num += x;
            add_num(5);
        }
        assert_eq!(10, num);
    }
    // コピーの所有権を取得する為、num自体は変わらない
    {
        let mut num = 5;
        {
            let mut add_num = move |x: i32| num += x;
            add_num(5);
        }
        assert_eq!(5, num);
    }

    // クロージャはトレイトのシュガーシンタックス

    // クロージャーを引数にとる
    fn call_with_one<F>(some_closure: F) -> i32
    where
        F: Fn(i32) -> i32,
    {
        some_closure(1)
    }

    let answer = call_with_one(|x| x + 2);
    assert_eq!(3, answer);

    // 関数ポインタとクロージャ
    fn add_one(i: i32) -> i32 {
        i + 1
    }
    let f = add_one;
    let answer = call_with_one(&f);
    assert_eq!(2, answer);

    // クロージャを返す
    fn factory() -> Box<Fn(i32) -> i32> {
        let num = 5;

        Box::new(move |x| x + num)
    }

    let f = factory();

    let answer = f(1);
    assert_eq!(6, answer);
}

fn main() {
    clojure_test();
    println!("Hello, world!");
}
