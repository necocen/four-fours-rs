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
pub use unary_op::*;

type Knowledge = HashMap<WrappedValue, Equation>;

#[derive(Debug, Copy, Clone)]
pub struct WrappedValue(Value);

impl WrappedValue {
    fn key(&self) -> u64 {
        unsafe { std::mem::transmute(self.0) }
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

    pub fn knowledge(&self, numbers: impl Into<String>) -> Knowledge {
        let key: String = numbers.into();
        let mut knowledge = HashMap::<WrappedValue, Equation>::new();

        // 数値単独での表現
        let e = Equation::from_numbers(&key);
        knowledge.insert(WrappedValue(e.value), e);

        // 部分列を二項演算で結合する
        for k in 1..key.len() {
            // NOTE: 本当はメモ化したいが、所有権の問題からメモを取り回せない（あるいはcloneが必要になる）ので断念
            let knowledge_left = self.knowledge(&key[0..k]);
            let knowledge_right = self.knowledge(&key[k..key.len()]);
            for op in self.binary_ops.iter() {
                for (_, e1) in knowledge_left.iter() {
                    for (_, e2) in knowledge_right.iter() {
                        match Equation::apply_binary(e1, e2, op) {
                            Some(equation) => match knowledge.entry(WrappedValue(equation.value)) {
                                Entry::Occupied(mut o) => {
                                    if o.get().cost > equation.cost {
                                        o.insert(equation);
                                    }
                                }
                                Entry::Vacant(v) => {
                                    v.insert(equation);
                                }
                            },
                            None => {}
                        }
                    }
                }
            }
        }

        // 単項演算で拡大する（３回まで）
        for _ in 0..3 {
            let prev_knowledge = knowledge.clone();
            for op in self.unary_ops.iter() {
                for (_, e) in prev_knowledge.iter() {
                    match Equation::apply_unary(e, op) {
                        Some(equation) => match knowledge.entry(WrappedValue(equation.value)) {
                            Entry::Occupied(mut o) => {
                                if o.get().cost > equation.cost {
                                    o.insert(equation);
                                }
                            }
                            Entry::Vacant(v) => {
                                v.insert(equation);
                            }
                        },
                        None => {}
                    }
                }
            }
        }

        knowledge
    }
}
