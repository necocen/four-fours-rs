use super::{BinaryOp, Token, UnaryOp, Value};

#[derive(Debug)]
/// 等式に関する知識。トークン列の計算結果がある値と一致することを示す。
pub struct Equation {
    pub tokens: Vec<Token>,
    pub cost: u8,
    pub value: Value,
}

impl Equation {
    /// `Equation`に単項演算子を適用して新しい`Equation`を作成
    pub(super) fn apply_unary(e: &Equation, op: &UnaryOp) -> Option<Equation> {
        let value = op.apply(e.value)?;
        // tokens = e.tokens + [op.token]
        let mut tokens = e.tokens.clone();
        tokens.push(op.token);
        Some(Equation {
            tokens,
            cost: e.cost + op.cost,
            value,
        })
    }

    pub(super) fn apply_binary(e1: &Equation, e2: &Equation, op: &BinaryOp) -> Option<Equation> {
        let value = op.apply(e1.value, e2.value)?;
        // tokens = e1.tokens + e2.tokens + [op.token]
        let mut tokens = e1.tokens.clone();
        tokens.append(&mut e2.tokens.clone());
        tokens.push(op.token);
        Some(Equation {
            tokens,
            cost: e1.cost + e2.cost + op.cost,
            value,
        })
    }

    /// 探索数値列からその全体を唯一の値として持つ`Equation`を作成
    pub fn from_numbers(numbers: &String) -> Equation {
        let tokens: Vec<Token> = numbers
            .as_bytes()
            .iter()
            .map(|c| 0xf0 + (c - b'0'))
            .collect();
        let value: f64 = numbers.parse::<i32>().unwrap().into();
        Equation {
            tokens,
            cost: 0,
            value,
        }
    }
}
