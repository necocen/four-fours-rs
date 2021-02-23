use super::{Token, Value};

#[derive(Debug)]
pub struct BinaryOp {
    /// トークン
    pub token: Token,
    /// 適用コスト
    pub cost: u8,
    /// 適用
    pub apply: fn(Value, Value) -> Option<Value>,
}

impl BinaryOp {
    pub fn new(token: Token, cost: u8, apply: fn(Value, Value) -> Option<Value>) -> Self {
        BinaryOp { token, cost, apply }
    }

    pub(super) fn apply(&self, lhs: Value, rhs: Value) -> Option<Value> {
        (self.apply)(lhs, rhs)
    }
}
