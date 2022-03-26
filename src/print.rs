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

impl UnaryOpPrinter {
    pub fn new(
        token: Token,
        prefix: impl Into<String>,
        suffix: impl Into<String>,
        precedence: i32,
        paren: bool,
    ) -> UnaryOpPrinter {
        UnaryOpPrinter {
            token,
            prefix: prefix.into(),
            suffix: suffix.into(),
            precedence,
            paren,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BinaryOpPrinter {
    token: Token,
    prefix: String,
    infix: String,
    suffix: String,
    precedence: i32,
    /// 左結合性（A * B * C == (A * B) * C）
    left_associative: bool,
    /// 右結合性（A * B * C == A * (B * C)）
    right_associative: bool,
    /// 左の項に括弧が必要かどうか
    paren_left: bool,
    /// 右の項に括弧が必要かどうか
    paren_right: bool,
}

impl BinaryOpPrinter {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        token: Token,
        prefix: impl Into<String>,
        infix: impl Into<String>,
        suffix: impl Into<String>,
        precedence: i32,
        left_associative: bool,
        right_associative: bool,
        paren_left: bool,
        paren_right: bool,
    ) -> BinaryOpPrinter {
        BinaryOpPrinter {
            token,
            prefix: prefix.into(),
            infix: infix.into(),
            suffix: suffix.into(),
            precedence,
            left_associative,
            right_associative,
            paren_left,
            paren_right,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Printer {
    unary_ops: Vec<UnaryOpPrinter>,
    binary_ops: Vec<BinaryOpPrinter>,
    paren_left: String,
    paren_right: String,
}

impl Printer {
    pub fn new(
        unary_ops: Vec<UnaryOpPrinter>,
        binary_ops: Vec<BinaryOpPrinter>,
        paren_left: impl Into<String>,
        paren_right: impl Into<String>,
    ) -> Printer {
        Printer {
            unary_ops,
            binary_ops,
            paren_left: paren_left.into(),
            paren_right: paren_right.into(),
        }
    }

    pub fn print(&self, equation: &Equation) -> String {
        // 部分式文字列と、最後に追加された演算子の優先度のペアのスタック
        let mut stack = Vec::<(String, i32)>::new();

        for token in equation.tokens.iter() {
            match token {
                0xe0..=0xef => {
                    // 数値一桁目
                    let n = token - 0xe0;
                    stack.push((n.to_string(), 0));
                }
                0xf0..=0xff => {
                    // 数値二桁目以降
                    let n = token - 0xf0;
                    let last_val = stack.pop().unwrap().0;
                    stack.push((last_val + &n.to_string(), 0));
                }
                _ => {
                    if let Some(op) = self.unary_ops.iter().find(|op| op.token == *token) {
                        // 単項演算子
                        let expr = stack.pop().unwrap();
                        // 括弧が必要な場合は括弧をつける
                        let expr = if op.paren && expr.1 >= op.precedence {
                            self.paren_left.clone() + &expr.0 + &self.paren_right
                        } else {
                            expr.0.clone()
                        };
                        // prefix + expr + suffixをpush
                        stack.push((op.prefix.clone() + &expr + &op.suffix, op.precedence));
                    } else if let Some(op) = self.binary_ops.iter().find(|op| op.token == *token) {
                        // 二項演算子
                        let expr2 = stack.pop().unwrap();
                        let expr1 = stack.pop().unwrap();
                        // 括弧が必要な場合は括弧をつける（左側オペランド）
                        let expr1 = if op.paren_left
                            && (expr1.1 > op.precedence
                                || expr1.1 == op.precedence && !op.left_associative)
                        {
                            self.paren_left.clone() + &expr1.0 + &self.paren_right
                        } else {
                            expr1.0.clone()
                        };
                        // 括弧が必要な場合は括弧をつける（右側オペランド）
                        let expr2 = if op.paren_right
                            && (expr2.1 > op.precedence
                                || expr2.1 == op.precedence && !op.right_associative)
                        {
                            self.paren_left.clone() + &expr2.0 + &self.paren_right
                        } else {
                            expr2.0.clone()
                        };
                        // prefix + expr1 + infix + expr2 + suffixをpush
                        stack.push((
                            op.prefix.clone() + &expr1 + &op.infix + &expr2 + &op.suffix,
                            op.precedence,
                        ));
                    } else {
                        panic!("Unexpected token appeared: {}", token);
                    }
                }
            }
        }

        assert!(stack.len() == 1);
        stack[0].0.to_string()
    }
}
