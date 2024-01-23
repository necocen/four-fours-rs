#[cfg(target_arch = "wasm32")]
extern crate wasm_bindgen;

pub mod print;
pub mod search;
#[cfg(target_arch = "wasm32")]
pub mod wasm;

pub use print::*;
use search::{BinaryOp, Equation, Knowledge, Searcher, UnaryOp};
use std::collections::{hash_map::Entry, HashMap};
#[cfg(all(target_arch = "wasm32", feature = "with-rayon"))]
pub use wasm_bindgen_rayon::init_thread_pool;

pub fn search_int(
    u_ops: Vec<UnaryOp>,
    b_ops: Vec<BinaryOp>,
    numbers: &str,
) -> HashMap<i32, Equation> {
    // 探索
    let searcher = Searcher::new(u_ops, b_ops);
    let mut memo = HashMap::<String, Knowledge>::default();
    searcher.search(&mut memo, numbers);
    let knowledge = &memo.get(numbers).unwrap();

    // 整数値のみを出力する
    let mut results = HashMap::<i32, Equation>::new();
    for r in knowledge.iter() {
        cfg_if::cfg_if! {
            if #[cfg(feature = "with-rayon")] {
                let e = r.value();
            } else {
                let (_, e) = r;
            }
        }
        if e.value >= 0f64 && e.value < 2000f64 && e.value.fract().abs() < 1e-9 {
            let rounded = e.value.round() as i32;
            match results.entry(rounded) {
                Entry::Occupied(mut o) => {
                    if o.get().cost > e.cost {
                        o.insert(e.clone());
                    }
                }
                Entry::Vacant(v) => {
                    v.insert(e.clone());
                }
            }
        }
    }
    results
}
