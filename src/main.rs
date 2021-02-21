mod search;
use search::{BinaryOp, UnaryOp, Searcher};

fn main() {
    println!("Hello, world!");

    //let negate = UnaryOp::new(0, 0, false, |v| -v);
    let add = BinaryOp::new(1, 0, false, false, |v1, v2| v1 + v2);
    let sub = BinaryOp::new(2, 1, false, false, |v1, v2| v1 - v2);
    let mul = BinaryOp::new(3, 0, false, false, |v1, v2| v1 * v2);
    let div = BinaryOp::new(4, 1, false, false, |v1, v2| v1 / v2);
    let searcher = Searcher::new(vec![add, sub, mul, div]);
    let knowledge = searcher.knowledge("4444");
    for (_, k) in knowledge.iter() {
        println!("{:?}, {:?}", k.value, k.tokens);
    }
}
