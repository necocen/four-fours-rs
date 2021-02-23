mod print;
mod search;
use print::{BinaryOpPrinter, Printer, UnaryOpPrinter};
use search::{BinaryOp, Equation, Searcher, UnaryOp};
use std::collections::{hash_map::Entry, HashMap};

fn main() {
    // unary ops
    let negate = UnaryOp::new(0x00, 2, |v| Some(-v));
    let sqrt = UnaryOp::new(0x01, 4, |v| if v < 0f64 { None } else { Some(v.sqrt()) });
    // binary ops
    let add = BinaryOp::new(0x10, 1, |v1, v2| Some(v1 + v2));
    let sub = BinaryOp::new(0x11, 2, |v1, v2| Some(v1 - v2));
    let mul = BinaryOp::new(0x12, 3, |v1, v2| Some(v1 * v2));
    let div = BinaryOp::new(
        0x13,
        4,
        |v1, v2| if v2 == 0f64 { None } else { Some(v1 / v2) },
    );
    let pow = BinaryOp::new(0x14, 6, |v1, v2| Some(v1.powf(v2)));

    // 探索
    let searcher = Searcher::new(vec![negate, sqrt], vec![add, sub, mul, div, pow]);
    let knowledge = searcher.knowledge("4444");

    // 整数値のみを出力する
    let mut results = HashMap::<i32, Equation>::new();
    for (_, e) in knowledge {
        if e.value >= 0f64 && e.value < 2000f64 && e.value.fract().abs() < 1e-9 {
            let rounded = e.value.round() as i32;
            match results.entry(rounded) {
                Entry::Occupied(mut o) => {
                    if o.get().cost > e.cost {
                        o.insert(e);
                    }
                }
                Entry::Vacant(v) => {
                    v.insert(e);
                }
            }
        }
    }

    // 結果表示
    let negate_p = UnaryOpPrinter::new(0x00, "-", "", 3, true);
    let sqrt_p = UnaryOpPrinter::new(0x01, "√", "", 1, true);
    let add_p = BinaryOpPrinter::new(0x10, "", "+", "", 6, true, true, true, true);
    let sub_p = BinaryOpPrinter::new(0x11, "", "-", "", 6, true, false, true, true);
    let mul_p = BinaryOpPrinter::new(0x12, "", "*", "", 5, true, true, true, true);
    let div_p = BinaryOpPrinter::new(0x13, "", "/", "", 5, true, false, true, true);
    let pow_p = BinaryOpPrinter::new(0x14, "", "^", "", 3, false, true, true, true);
    let printer = Printer::new(
        vec![negate_p, sqrt_p],
        vec![add_p, sub_p, mul_p, div_p, pow_p],
        "(",
        ")",
        " = ",
    );

    for n in 0..=1000 {
        if let Some(e) = results.get(&n) {
            println!("{}", printer.print(e));
        }
    }
}
