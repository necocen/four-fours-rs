use super::{Token, Value};

#[derive(Debug)]
pub struct UnaryOp<F: Fn(Value) -> Value> {
    /// トークン
    pub token: Token,
    /// 適用コスト
    pub cost: u8,
    /// 整数値にのみ適用可能
    pub int_only: bool,
    /// 適用
    pub apply: F,
}

impl<F: Fn(Value) -> Value> UnaryOp<F> {
    pub fn new(token: Token, cost: u8, int_only: bool, apply: F) -> Self {
        UnaryOp {
            token,
            cost,
            int_only,
            apply,
        }
    }

    pub(super) fn apply(&self, value: Value) -> Option<Value> {
        if self.int_only && value.fract().abs() > f64::MIN_POSITIVE {
            return None;
        }
        Some((self.apply)(value))
    }
}
