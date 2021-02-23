mod search;
use search::{BinaryOp, UnaryOp, Searcher};

fn main() {
    println!("Hello, world!");

    let negate = UnaryOp::new(0x00, 1, false, |v| -v);
    let add = BinaryOp::new(0x10, 0, false, false, |v1, v2| v1 + v2);
    let sub = BinaryOp::new(0x11, 1, false, false, |v1, v2| v1 - v2);
    let mul = BinaryOp::new(0x12, 0, false, false, |v1, v2| v1 * v2);
    let div = BinaryOp::new(0x13, 1, false, false, |v1, v2| v1 / v2);
    let searcher = Searcher::new(vec![negate], vec![add, sub, mul, div]);
    let knowledge = searcher.knowledge("1234");
    for (_, k) in knowledge.iter() {
        println!("{:?}, {:?}", k.value, k.tokens);
    }
}
