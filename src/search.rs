mod binary_op;
mod equation;
mod unary_op;

pub type Value = f64;
/// 演算子および桁トークン。
/// `0xf0`より後の値は桁のために予約済みで、
/// `t = 0xf0 + n (0 <= n < 10)`のとき、数字`n`を表す。
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
    knowledge: HashMap<String, Knowledge>,
    unary_ops: Vec<UnaryOp>,
    binary_ops: Vec<BinaryOp>,
}

impl Searcher {
    pub fn new(unary_ops: Vec<UnaryOp>, binary_ops: Vec<BinaryOp>) -> Searcher {
        Searcher {
            knowledge: HashMap::new(),
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
            // TODO: この_knowledgeは必ずメモ化されているようにできるか？（これは参照を返さなければならないはずだが、
            // この時点での計算になると&mut selfが必要になるのでborrowできない）
            // つまり、再帰で計算してしまうとうまくいかない。ボトムアップに計算する
            let knowledge_left = self.knowledge(&key[0..k]);
            let knowledge_right = self.knowledge(&key[k..key.len()]);
            for (_, e1) in knowledge_left.iter() {
                for (_, e2) in knowledge_right.iter() {
                    for op in self.binary_ops.iter() {
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

        let b_knowledge = knowledge;
        let mut knowledge = HashMap::<WrappedValue, Equation>::new();

        // 単項演算で拡大する（２回まで）
        for _ in 0..2 {
            for op in self.unary_ops.iter() {
                for (_, e) in b_knowledge.iter() {
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
