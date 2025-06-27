use std::fmt::{self, Display};

/// Represents the set of mathematical operators that can appear in an expression.
/// Each variant is a binary operator that accepts two operands.
#[derive(Debug, PartialEq,Clone)]
pub enum Op {
    Add,    // '+'
    Sub,    // '-'
    Mul,    // '*'
    Div,    // '/'
    Pow,    // '^'
}

/// Represents a node within the Abstract Syntax Tree (AST).
/// The AST is a tree-like data structure that mirrors the structure of a mathematical expression.
#[derive(Debug, PartialEq,Clone)]
pub enum Expr {
    Num(f64),
    /// A variable, such as `x` or `y`.
    Var(String),
    /// A binary operation, composed of an operator and two operand expressions.
    /// For example, `x + 2` would be represented as `BinaryOp { op: Op::Add, ... }`.
    BinaryOp {
        op: Op,
        left: Box<Expr>,
        right: Box<Expr>,
    },
     Func(String,Box<Expr>),
     

}

/// Implements the `Display` trait to define how an `Op` is converted to a string.
/// This is used for printing the AST in a human-readable format.
impl Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Op::Add => write!(f, "+"),
            Op::Sub => write!(f, "-"),
            Op::Mul => write!(f, "*"),
            Op::Div => write!(f, "/"),
            Op::Pow => write!(f, "^"),
        }
    }
}

/// Implements the `Display` trait to define how an `Expr` is converted to a string.
/// This enables the entire expression tree to be printed in a readable format.
impl Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Num(n) => write!(f, "{}", n),
            Expr::Var(s) => write!(f, "{}", s),
            Expr::BinaryOp { op, left, right } => {
                write!(f, "({} {} {})", left, op, right)
            }
            Expr::Func(name,arg) => {
                write!(f, "{} {}", name,arg)
            }
        }
    }
}
