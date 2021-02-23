mod search;
use search::{BinaryOp, Searcher, UnaryOp};

fn main() {
    println!("Hello, world!");

    let negate = UnaryOp::new(0x00, 2, |v| Some(-v));
    let sqrt = UnaryOp::new(0x01, 4, |v| if v < 0f64 { None } else { Some(v.sqrt()) });
    let add = BinaryOp::new(0x10, 1, |v1, v2| Some(v1 + v2));
    let sub = BinaryOp::new(0x11, 2, |v1, v2| Some(v1 - v2));
    let mul = BinaryOp::new(0x12, 3, |v1, v2| Some(v1 * v2));
    let div = BinaryOp::new(
        0x13,
        4,
        |v1, v2| if v2 == 0f64 { None } else { Some(v1 / v2) },
    );
    let pow = BinaryOp::new(0x14, 6, |v1, v2| Some(v1.powf(v2)));
    let searcher = Searcher::new(vec![negate, sqrt], vec![add, sub, mul, div, pow]);
    let knowledge = searcher.knowledge("1234");

    for (_, k) in knowledge.iter() {
        // 整数値のみ出力
        if k.value.fract().abs() < 1e-9 {
            println!("{:?}, {:?}", k.value, k.tokens);
        }
    }
}
