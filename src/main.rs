mod search;
use search::{BinaryOp, Searcher, UnaryOp};

fn main() {
    println!("Hello, world!");

    let negate = UnaryOp::new(0x00, 1, |v| Some(-v));
    let sqrt = UnaryOp::new(0x01, 2, |v| if v < 0f64 { None } else { Some(v.sqrt()) });
    let add = BinaryOp::new(0x10, 0, |v1, v2| Some(v1 + v2));
    let sub = BinaryOp::new(0x11, 1, |v1, v2| Some(v1 - v2));
    let mul = BinaryOp::new(0x12, 0, |v1, v2| Some(v1 * v2));
    let div = BinaryOp::new(
        0x13,
        1,
        |v1, v2| if v2 == 0f64 { None } else { Some(v1 / v2) },
    );
    let searcher = Searcher::new(vec![negate, sqrt], vec![add, sub, mul, div]);
    let knowledge = searcher.knowledge("1234");
    for (_, k) in knowledge.iter() {
        println!("{:?}, {:?}", k.value, k.tokens);
    }
}
