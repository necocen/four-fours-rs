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
use std::{
    collections::{hash_map::Entry, HashMap},
    hash,
};

pub use binary_op::*;
pub use equation::*;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
pub use unary_op::*;

pub type Knowledge = HashMap<WrappedValue, Equation>;

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
        log::info!("Start searching for {numbers}");
        let mut knowledge = HashMap::<WrappedValue, Equation>::new();
        // 数値単独での表現
        let e = Equation::from_numbers(numbers);
        knowledge.insert(WrappedValue(e.value), e);

        for i in 1..numbers.len() {
            let (key_left, key_right) = numbers.split_at(i);
            log::info!("Start combining {key_left} and {key_right}");
            self.search(memo, key_left);
            self.search(memo, key_right);
            let knowledge_left = &memo[key_left];
            let knowledge_right = &memo[key_right];
            let combined = self
                .binary_ops
                .par_iter()
                .flat_map(|op| {
                    knowledge_left.par_iter().flat_map(|(_, e1)| {
                        knowledge_right
                            .par_iter()
                            .filter_map(|(_, e2)| Equation::apply_binary(e1, e2, op))
                    })
                })
                .collect::<Vec<_>>();
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
        }

        // 単項演算で拡大する（３回まで）
        for i in 0..3 {
            log::info!("Start applying unary ops to {numbers} - {}", i + 1);
            let applied = self
                .unary_ops
                .par_iter()
                .flat_map(|op| {
                    knowledge
                        .par_iter()
                        .filter_map(|(_, e)| Equation::apply_unary(e, op))
                })
                .collect::<Vec<_>>();
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

        log::info!("End searching for {numbers}");
        memo.insert(numbers.to_string(), knowledge);
    }
}
