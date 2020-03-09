// 関連型

// trait Graph<N,E> {
//     fn has_edge(&self, &N,&N) -> bool;
//     fn edges(&self, &N) -> Vec<E>;
//     // etc
// }

// fn distance<N,E,G:Graph<N,E>>(graph: &G,start:&N,end:&N) -> u32 {
// // 
// };

// なぜかコンパイルエラーが起こる、原因不明
trait Graph {
    // この関連型によってGraphというトレイトがNとEを含んでいるということのみを表現できるようになった。
    // 関連型を使う時はtype宣言を使用する。
    type N;
    type E;

    fn has_edge(&self, &Self::N, &Self::N) -> bool;
    fn edges(&self, &Self::N) -> Vec<Self::E>;
}

fn distance<G: Graph>(graph: &G, start: &G::N,end: &G::N) -> u32 {
    8u32
}


// 関連型を実装する

struct Node;

struct Edge;

struct MyGraph;

impl Graph for MyGraph {
    type N = Node;
    type E = Edge;

    fn has_edge(&self, n1: &Node, n2: &Node) -> bool {
        true
    }

    fn edges(&self, n: &Node) -> Vec<Edge> {
        Vec::new()
    }
}

fn main() {
    println!("Hello, world!");
}
