struct HasDrop;

// Dropはスコープの外に行った時に実行される
// スタックの原理に従ってDropが実行されるのはFiLO
impl Drop for HasDrop {
    fn drop(&mut self) {
        println!("Dropping");
    }
}

fn main() {
    println!("start");
    {
        let x = HasDrop;
        println!("processing");
    }
    println!("finish");
}
