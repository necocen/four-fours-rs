use super::{Token, Value};

#[derive(Debug)]
pub struct BinaryOp<F: Fn(Value, Value) -> Value> {
    /// トークン
    pub token: Token,
    /// 適用コスト
    pub cost: u8,
    /// 左辺が整数値にのみ適用可能
    pub int_only_lhs: bool,
    /// 右辺が整数値にのみ適用可能
    pub int_only_rhs: bool,
    /// 適用
    pub apply: F,
}

impl<F: Fn(Value, Value) -> Value> BinaryOp<F> {
    pub fn new(token: Token, cost: u8, int_only_lhs: bool, int_only_rhs: bool, apply: F) -> Self {
        BinaryOp {
            token,
            cost,
            int_only_lhs,
            int_only_rhs,
            apply,
        }
    }

    pub(super) fn apply(&self, lhs: Value, rhs: Value) -> Option<Value> {
        if self.int_only_lhs && lhs.fract().abs() > f64::MIN_POSITIVE {
            return None;
        }
        if self.int_only_rhs && rhs.fract().abs() > f64::MIN_POSITIVE {
            return None;
        }

        Some((self.apply)(lhs, rhs))
    }
}
