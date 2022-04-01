use crate::{
    print::{BinaryOpPrinter, Printer, UnaryOpPrinter},
    search::{BinaryOp, UnaryOp},
    search_int,
};
use js_sys::Map;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = "initLog")]
pub fn init_log() {
    wasm_logger::init(wasm_logger::Config::default());
}

#[wasm_bindgen(js_name = "searchWasm")]
pub fn search_wasm(numbers: &str) -> Map {
    let map = Map::new();
    // 演算子
    let negate = UnaryOp::new(0x00, 2, |v| Some(-v));
    let sqrt = UnaryOp::new(0x01, 4, |v| match v {
        x if x > 0f64 && x != 0f64 && x != 1f64 => Some(v.sqrt()),
        _ => None,
    });
    let fact = UnaryOp::new(0x02, 6, |v| {
        const TABLE: [f64; 9] = [1., 1., 2., 6., 24., 120., 720., 5040., 40320.];
        if v < 0f64 || v > (i32::MAX as f64) || v.fract().abs() > f64::EPSILON {
            return None;
        }
        let n = v.round() as usize;
        if n >= TABLE.len() {
            return None;
        }
        Some(TABLE[n])
    });
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

    // 結果表示
    let negate_p = UnaryOpPrinter::new(0x00, "-", "", 3, true);
    let sqrt_p = UnaryOpPrinter::new(0x01, "√", "", 1, true);
    let fact_p = UnaryOpPrinter::new(0x02, "", "!", 2, true);
    let add_p = BinaryOpPrinter::new(0x10, "", "+", "", 6, true, true, true, true);
    let sub_p = BinaryOpPrinter::new(0x11, "", "-", "", 6, true, false, true, true);
    let mul_p = BinaryOpPrinter::new(0x12, "", "*", "", 5, true, true, true, true);
    let div_p = BinaryOpPrinter::new(0x13, "", "/", "", 5, true, false, true, true);
    let pow_p = BinaryOpPrinter::new(0x14, "", "^", "", 3, false, true, true, true);
    let printer = Printer::new(
        vec![negate_p, sqrt_p, fact_p],
        vec![add_p, sub_p, mul_p, div_p, pow_p],
        "(",
        ")",
    );
    search_int(
        vec![negate, sqrt, fact],
        vec![add, sub, mul, div, pow],
        numbers,
    )
    .into_iter()
    .map(|(n, result)| (n, printer.print(&result)))
    .for_each(|(n, result)| {
        map.set(&JsValue::from(n), &JsValue::from(result));
    });
    map
}
