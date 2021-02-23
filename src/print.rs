use super::search::{Equation, Token};

#[derive(Debug, Clone)]
pub struct UnaryOpPrinter {
    token: Token,
    prefix: String,
    suffix: String,
    /// 演算子優先度
    precedence: i32,
    /// 括弧が必要？
    paren: bool,
}

#[derive(Debug, Clone)]
pub struct BinaryOpPrinter {
    token: Token,
    prefix: String,
    infix: String,
    suffix: String,
    precedence: i32,
    left_associative: bool,
    right_associative: bool,
    paren_lhs: bool,
    paren_rhs: bool,
}

#[derive(Debug, Clone)]
pub struct Printer {
    unary_ops: Vec<UnaryOpPrinter>,
    binary_ops: Vec<BinaryOpPrinter>,
    paren_left: String,
    paren_right: String,
    equal: String,
}

impl Printer {
    pub fn new(
        unary_ops: Vec<UnaryOpPrinter>,
        binary_ops: Vec<BinaryOpPrinter>,
        paren_left: impl Into<String>,
        paren_right: impl Into<String>,
        equal: impl Into<String>,
    ) -> Printer {
        Printer {
            unary_ops,
            binary_ops,
            paren_left: paren_left.into(),
            paren_right: paren_right.into(),
            equal: equal.into(),
        }
    }

    pub fn print(&self, equation: &Equation) -> String {
        let n = equation.value.round() as i32;
        format!(
            "{}{}{:?} c:{}",
            n, self.equal, equation.tokens, equation.cost
        )
    }
}
