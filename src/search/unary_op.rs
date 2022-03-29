use super::{Token, Value};

#[derive(Debug, Clone, Copy)]
pub struct UnaryOp {
    /// トークン
    pub token: Token,
    /// 適用コスト
    pub cost: u8,
    /// 適用
    pub apply: fn(Value) -> Option<Value>,
}

impl UnaryOp {
    pub fn new(token: Token, cost: u8, apply: fn(Value) -> Option<Value>) -> Self {
        UnaryOp { token, cost, apply }
    }

    pub(super) fn apply(&self, value: Value) -> Option<Value> {
        (self.apply)(value)
    }
}
