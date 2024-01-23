mod binary_op;
mod equation;
mod unary_op;

pub type Value = f64;
/// 演算子および桁トークン。
/// `0xe0`より後の値は桁のために予約済みで、
/// - `t = 0xe0 + n (0 <= n < 10)`
/// または
/// - `t = 0xf0 + n (0 <= n < 10)`
///
/// のとき、数字`n`を表す。
/// `0xeX`は先頭桁を示し、`0xfX`は残りの桁を示す。
pub type Token = u8;
#[cfg(not(feature = "with-rayon"))]
use std::collections::hash_map::Entry;
use std::{collections::HashMap, hash};

pub use binary_op::*;

#[cfg(feature = "with-rayon")]
use dashmap::{mapref::entry::Entry, DashMap};
pub use equation::*;
use fnv::FnvBuildHasher;
#[cfg(feature = "with-rayon")]
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
pub use unary_op::*;

#[cfg(feature = "with-rayon")]
pub type Knowledge = DashMap<WrappedValue, Equation, FnvBuildHasher>;
#[cfg(not(feature = "with-rayon"))]
pub type Knowledge = HashMap<WrappedValue, Equation, FnvBuildHasher>;

#[derive(Debug, Copy, Clone)]
pub struct WrappedValue(Value);

impl WrappedValue {
    fn key(&self) -> u64 {
        self.0.to_bits()
    }
}

impl hash::Hash for WrappedValue {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.key().hash(state)
    }
}

impl PartialEq for WrappedValue {
    fn eq(&self, other: &WrappedValue) -> bool {
        self.key() == other.key()
    }
}

impl Eq for WrappedValue {}

pub struct Searcher {
    unary_ops: Vec<UnaryOp>,
    binary_ops: Vec<BinaryOp>,
}

impl Searcher {
    pub fn new(unary_ops: Vec<UnaryOp>, binary_ops: Vec<BinaryOp>) -> Searcher {
        Searcher {
            unary_ops,
            binary_ops,
        }
    }

    pub fn search(&self, memo: &mut HashMap<String, Knowledge>, numbers: &str) {
        if memo.contains_key(numbers) {
            return;
        }
        log::info!("Start searching for {}", numbers);

        #[cfg(feature = "with-rayon")]
        let knowledge = Knowledge::default();
        #[cfg(not(feature = "with-rayon"))]
        let mut knowledge = Knowledge::default();

        // 数値単独での表現
        let e = Equation::from_numbers(numbers);
        knowledge.insert(WrappedValue(e.value), e);

        for i in 1..numbers.len() {
            let (key_left, key_right) = numbers.split_at(i);
            self.search(memo, key_left);
            self.search(memo, key_right);
        }

        log::debug!("Combining...");
        cfg_if::cfg_if! {
            if #[cfg(feature = "with-rayon")] {
                let combined =  (1..numbers.len())
                    .into_par_iter()
                    .flat_map(|i| {
                        let (key_left, key_right) = numbers.split_at(i);
                        let knowledge_left = &memo[key_left];
                        let knowledge_right = &memo[key_right];
                        self.binary_ops.par_iter().flat_map(move |op| {
                            knowledge_left.par_iter().flat_map(move |r1| {
                                knowledge_right
                                    .par_iter()
                                    .filter_map(move |r2| Equation::apply_binary(r1.value(), r2.value(), op))
                            })
                        })
                    })
                    .collect::<Vec<_>>();
                log::debug!("Merging...");
                for equation in combined {
                    match knowledge.entry(WrappedValue(equation.value)) {
                        Entry::Occupied(mut o) => {
                            if o.get().cost > equation.cost {
                                o.insert(equation);
                            }
                        }
                        Entry::Vacant(v) => {
                            v.insert(equation);
                        }
                    }
                }
            } else {
                (1..numbers.len())
                    .flat_map(|i| {
                        let (key_left, key_right) = numbers.split_at(i);
                        let knowledge_left = &memo[key_left];
                        let knowledge_right = &memo[key_right];
                        self.binary_ops.iter().flat_map(move |op| {
                            knowledge_left.iter().flat_map(move |(_, e1)| {
                                knowledge_right
                                    .iter()
                                    .filter_map(move |(_, e2)| Equation::apply_binary(e1, e2, op))
                            })
                        })
                    }).for_each(|equation| {
                        match knowledge.entry(WrappedValue(equation.value)) {
                            Entry::Occupied(mut o) => {
                                if o.get().cost > equation.cost {
                                    o.insert(equation);
                                }
                            }
                            Entry::Vacant(v) => {
                                v.insert(equation);
                            }
                        }
                    })
            }
        }

        // 単項演算で拡大する（３回まで）
        for i in 0..3 {
            log::info!("Start applying unary ops to {} - {}", numbers, i + 1);
            cfg_if::cfg_if! {
                if #[cfg(feature = "with-rayon")] {
                    let applied = self.unary_ops
                    .par_iter()
                    .flat_map(|op| {
                        knowledge
                            .par_iter()
                            .filter_map(move |r| Equation::apply_unary(r.value(), op))
                    })
                    .collect::<Vec<_>>();
                } else {
                    let applied = self.unary_ops
                    .iter()
                    .flat_map(|op| {
                        knowledge
                            .iter()
                            .filter_map(move |(_, e)| Equation::apply_unary(e, op))
                    })
                    .collect::<Vec<_>>();
                }
            }
            log::debug!("Merging...");
            for equation in applied {
                match knowledge.entry(WrappedValue(equation.value)) {
                    Entry::Occupied(mut o) => {
                        if o.get().cost > equation.cost {
                            o.insert(equation);
                        }
                    }
                    Entry::Vacant(v) => {
                        v.insert(equation);
                    }
                }
            }
        }

        log::info!("End searching for {}", numbers);
        memo.insert(numbers.to_string(), knowledge);
    }
}
