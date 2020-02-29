// ベクタ: 拡張可能な配列

// 普通の配列は
// let list1 = [1,2,3]; // list1: [i32, 3] //長さを変えることはできない

fn vector_test() {
    let vect = vec![1, 2, 3, 4, 5]; // vect: Vec<i32>
    let vect2 = vec![0; 10]; // 0が10個

    // vect.push(6); // Error!: letで宣言されていてimmutableだからpushできない

    // 要素へのアクセス
    println!("vect[1]: {}", vect[1]);

    // インデックスの型はusize
    let index1: usize = 0;
    let index_error: i32 = 0;

    let index_zero1 = vect[index1];
    // let index_zero_error = vect[index_error] //Error! インデックスは型がusizeでなければいけない

    let mut vect3 = vec![1, 2, 3];

    vect3.push(4);

    // イテレーティング
    for i in &vect3 {
        println!("A reference to {}", i);
    }

    for i in &mut vect3 {
        println!("A mutable reference to {}", i);
    }

    for i in vect3 {
        println!("Take ownership of the vector and its element {}", i);
    }
}

fn main() {
    vector_test();
    println!("Hello, world!");
}

// ベクタは拡張可能な配列
