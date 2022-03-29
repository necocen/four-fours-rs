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
use rayon::{
    iter::{once, IntoParallelIterator, IntoParallelRefIterator, ParallelBridge, ParallelIterator},
    slice::ParallelSliceMut,
};
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
        log::info!("Start searching for {}", numbers);

        for i in 1..numbers.len() {
            let (key_left, key_right) = numbers.split_at(i);
            self.search(memo, key_left);
            self.search(memo, key_right);
        }

        log::info!("Start applying binary ops to {}", numbers);
        log::debug!("Combining...");
        let mut equations = (1..numbers.len())
            .map(|i| {
                let (key_left, key_right) = numbers.split_at(i);
                let knowledge_left = &memo[key_left];
                let knowledge_right = &memo[key_right];
                let combined = self.binary_ops.par_iter().flat_map(move |op| {
                    knowledge_left.par_iter().flat_map(move |(_, e1)| {
                        knowledge_right
                            .par_iter()
                            .filter_map(move |(_, e2)| Equation::apply_binary(e1, e2, op))
                    })
                });
                combined
            })
            .par_bridge()
            .flatten()
            .chain(once(Equation::from_numbers(numbers)))
            .collect::<Vec<_>>();
        log::debug!("Sorting...");
        equations.par_sort_by_key(|e| -(e.cost as i32));
        log::debug!("Merging...");
        let mut knowledge = equations
            .into_par_iter()
            .map(|e| (WrappedValue(e.value), e))
            .collect::<HashMap<_, _>>();

        // 単項演算で拡大する（３回まで）
        for i in 0..3 {
            log::info!("Start applying unary ops to {} - {}", numbers, i + 1);
            let equations = knowledge
                .par_iter()
                .flat_map(|(_, e)| {
                    self.unary_ops
                        .par_iter()
                        .filter_map(move |op| Equation::apply_unary(e, op))
                })
                .collect::<Vec<_>>();
            for equation in equations {
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
