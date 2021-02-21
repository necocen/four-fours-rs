use super::Value;

pub struct BinaryOp<F: Fn(Value, Value) -> Value> {
    /// 適用コスト
    cost: u8,
    /// 左辺が整数値にのみ適用可能
    int_only_lhs: bool,
    /// 右辺が整数値にのみ適用可能
    int_only_rhs: bool,
    /// 適用
    apply: F,
}

impl<F: Fn(Value, Value) -> Value> BinaryOp<F> {
    pub fn new(cost: u8, int_only_lhs: bool, int_only_rhs: bool, apply: F) -> Self {
        BinaryOp {
            cost,
            int_only_lhs,
            int_only_rhs,
            apply,
        }
    }
}
