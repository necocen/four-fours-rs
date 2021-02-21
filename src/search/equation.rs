use super::{BinaryOp, Token, UnaryOp, Value};

#[derive(Debug)]
/// 等式に関する知識。トークン列の計算結果がある値と一致することを示す。
pub struct Equation {
    tokens: Vec<Token>,
    cost: u8,
    value: Value,
}

impl Equation {
    /// `Equation`に単項演算子を適用して新しい`Equation`を作成
    pub(super) fn apply_unary<F: Fn(Value) -> Value>(
        e: &mut Equation,
        op: &UnaryOp<F>,
    ) -> Option<Equation> {
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

    pub(super) fn apply_binary<F: Fn(Value, Value) -> Value>(
        e1: &mut Equation,
        e2: &mut Equation,
        op: &BinaryOp<F>,
    ) -> Option<Equation> {
        let value = op.apply(e1.value, e2.value)?;
        // tokens = e1.tokens + e2.tokens + [op.token]
        let mut tokens = e1.tokens.clone();
        tokens.append(&mut e2.tokens);
        tokens.push(op.token);
        Some(Equation {
            tokens,
            cost: e1.cost + e2.cost + op.cost,
            value,
        })
    }

    /// 値から`Equation`を作成
    pub fn from_value(value: Value, token: Token) -> Equation {
        Equation {
            tokens: vec![token],
            cost: 0,
            value,
        }
    }
}
