struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

// メソッド: implで実装して引数にselfのようなそれ自体を受け取るもの
impl Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }

    fn grow(&self, increment: f64) -> Circle {
        Circle {
            x: self.x,
            y: self.y,
            radius: self.radius + increment,
        }
    }
}

impl Circle {
    fn reference(&self) {
        // 参照を渡す
        println!("taking self by reference!");
    }
    fn mutable_reference(&mut self) {
        // 借用する
        println!("taking self by mutable reference!");
    }
    fn takes_ownership(self) {
        // 所有権を受け取る
        println!("taking ownership of self");
    }
}

// 関連関数: implで実装するselfなどそれ自体を受け取らないもの, 他の言語では静的メソッドと呼ばれる
impl Circle {
    fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle {
            x: x,
            y: y,
            radius: radius,
        }
    }
}

// Builderパターン: Rustでは名前付き引数や可変個引数といった機能がない、その代わりBuilderパターンを採用している
struct CircleBuilder {
    x: f64,
    y: f64,
    radius: f64,
}

impl CircleBuilder {
    fn new() -> CircleBuilder {
        // デフォルトの値を決める
        CircleBuilder {
            x: 0.0,
            y: 0.0,
            radius: 1.0,
        }
    }

    fn x(&mut self, coordinate: f64) -> &mut CircleBuilder {
        // 名前付き引数や可変個引数の代わりにx,y等のメソッドを作成する
        self.x = coordinate;
        self
    }

    fn y(&mut self, coordinate: f64) -> &mut CircleBuilder {
        self.y = coordinate;
        self
    }

    fn radius(&mut self, radius: f64) -> &mut CircleBuilder {
        self.radius = radius;
        self
    }
    fn finalize(&self) -> Circle {
        Circle {
            x: self.x,
            y: self.y,
            radius: self.radius,
        }
    }
}

fn main() {
    let c = Circle {
        x: 0.0,
        y: 0.0,
        radius: 2.0,
    };
    println!("{}", c.area());
    // メソッドチェーン
    let d = c.grow(2.0).area();
    println!("{}", d);

    // 関連関数は::でアクセスする
    let e = Circle::new(0.0, 0.0, 2.0);

    // Builderパターンの使い方
    let new_circle = CircleBuilder::new().x(2.0).y(2.0).radius(3.0).finalize();

    println!("new_circle_x{}", new_circle.x);
}
