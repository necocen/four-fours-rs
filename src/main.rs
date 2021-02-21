mod search;
use search::{BinaryOp, UnaryOp};

fn main() {
    println!("Hello, world!");

    let negate = UnaryOp::new(0, false, |v| -v);
    let add = BinaryOp::new(0, false, false, |v1, v2| v1 + v2);
}
