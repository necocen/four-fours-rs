use super::Value;

pub struct UnaryOp<F: Fn(Value) -> Value> {
    /// 適用コスト
    cost: u8,
    /// 整数値にのみ適用可能
    int_only: bool,
    /// 適用
    apply: F,
}

impl<F: Fn(Value) -> Value> UnaryOp<F> {
    pub fn new(cost: u8, int_only: bool, apply: F) -> Self {
        UnaryOp {
            cost,
            int_only,
            apply,
        }
    }
}
