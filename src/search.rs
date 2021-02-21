mod binary_op;
mod equation;
mod unary_op;

pub type Value = f64;
pub type Token = u8;
pub use binary_op::*;
pub use equation::*;
pub use unary_op::*;
