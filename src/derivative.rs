use crate::ast::{Expr, Op};
use Expr::*;
use Op::*;

use crate::function_table::conversion; // function_table contains derivative formulas for built-in funcs like sin, cos, etc.

/// Computes the derivative of an expression with respect to the given variable.
///
/// This is the main entry point for differentiation.
/// It dispatches to different rules depending on the Expr variant.
pub fn derivative(expr: &Expr, var: &str) -> Expr {
    match expr {
        Num(_) => constant_rule(), // derivative of a constant is zero
        Var(v) => variable_rule(v, var), // derivative of variable: 1 if it matches, else 0
        BinaryOp { op, left, right } => match op {
            Add => add_rule(left, right, var), // sum rule
            Sub => sub_rule(left, right, var), // difference rule
            Mul => product_rule(left, right, var), // product rule
            Div => quotient_rule(left, right, var), // quotient rule
            Pow => pow_rule(left, right, var), // power rule with chain rule
        },
        Func(name, arg) => func_rule(name, arg, var), // chain rule for functions like sin, cos
    }
}

/// Derivative of a constant is zero.
fn constant_rule() -> Expr {
    Num(0.0)
}

/// Derivative of a variable:
/// returns 1 if the variable matches the differentiation variable, else 0.
fn variable_rule(v: &str, var: &str) -> Expr {
    if v == var {
        Num(1.0)
    } else {
        Num(0.0)
    }
}

/// Sum rule: derivative of f + g is f' + g'
fn add_rule(left: &Expr, right: &Expr, var: &str) -> Expr {
    BinaryOp {
        op: Add,
        left: Box::new(derivative(left, var)),
        right: Box::new(derivative(right, var)),
    }
}

/// Difference rule: derivative of f - g is f' - g'
fn sub_rule(left: &Expr, right: &Expr, var: &str) -> Expr {
    BinaryOp {
        op: Sub,
        left: Box::new(derivative(left, var)),
        right: Box::new(derivative(right, var)),
    }
}

/// Power rule with chain rule:
///
/// For d/dx [x^n] where x is the variable, returns n * x^(n-1).
///
/// For d/dx [f(x)^n], uses chain rule:
/// n * f(x)^(n-1) * f'(x)
fn pow_rule(left: &Expr, right: &Expr, var: &str) -> Expr {
    match right {
        Expr::Num(n) => match left {
            Expr::Var(v) if v == var => {
                // Simple power rule: x^n
                Expr::BinaryOp {
                    op: Op::Mul,
                    left: Box::new(Expr::Num(*n)),
                    right: Box::new(Expr::BinaryOp {
                        op: Op::Pow,
                        left: Box::new(Expr::Var(v.clone())),
                        right: Box::new(Expr::Num(n - 1.0)),
                    }),
                }
            }
            _ => {
                // General power rule: f(x)^n
                // Apply chain rule: n * f(x)^(n-1) * f'(x)
                let d_left = derivative(left, var);
                Expr::BinaryOp {
                    op: Op::Mul,
                    left: Box::new(Expr::Num(*n)),
                    right: Box::new(Expr::BinaryOp {
                        op: Op::Mul,
                        left: Box::new(Expr::BinaryOp {
                            op: Op::Pow,
                            left: Box::new(left.clone()),
                            right: Box::new(Expr::Num(n - 1.0)),
                        }),
                        right: Box::new(d_left),
                    }),
                }
            }
        },
        _ => panic!("Power rule only implemented for powers with constant exponents"),
    }
}

/// Product rule: d/dx [u * v] = u' * v + u * v'
fn product_rule(left: &Expr, right: &Expr, var: &str) -> Expr {
    let u_prime = derivative(left, var);
    let v_prime = derivative(right, var);

    BinaryOp {
        op: Add,
        left: Box::new(BinaryOp {
            op: Mul,
            left: Box::new(u_prime),
            right: Box::new(right.clone()),
        }),
        right: Box::new(BinaryOp {
            op: Mul,
            left: Box::new(left.clone()),
            right: Box::new(v_prime),
        }),
    }
}

/// Quotient rule: d/dx [u / v] = (u' * v - u * v') / v^2
fn quotient_rule(left: &Expr, right: &Expr, var: &str) -> Expr {
    let u_prime = derivative(left, var);
    let v_prime = derivative(right, var);

    let numerator = BinaryOp {
        op: Sub,
        left: Box::new(BinaryOp {
            op: Mul,
            left: Box::new(u_prime),
            right: Box::new(right.clone()),
        }),
        right: Box::new(BinaryOp {
            op: Mul,
            left: Box::new(left.clone()),
            right: Box::new(v_prime),
        }),
    };

    let denominator = BinaryOp {
        op: Pow,
        left: Box::new(right.clone()),
        right: Box::new(Num(2.0)),
    };

    BinaryOp {
        op: Div,
        left: Box::new(numerator),
        right: Box::new(denominator),
    }
}

/// Chain rule for functions:
///
/// Given f(g(x)), the derivative is f'(g(x)) * g'(x)
///
/// `conversion` is a helper function that returns the derivative expression
/// of the outer function f evaluated at g(x).
fn func_rule(name: &str, arg: &Expr, var: &str) -> Expr {
    // Derivative of the outer function f evaluated at g(x)
    let outer_derivative = conversion(name, arg.clone())
        .unwrap_or_else(|e| panic!("{}", e));

    // Derivative of the inner function g(x)
    let inner_derivative = derivative(arg, var);

    // Chain rule: multiply outer derivative by inner derivative
    BinaryOp {
        op: Mul,
        left: Box::new(outer_derivative),
        right: Box::new(inner_derivative),
    }
}
