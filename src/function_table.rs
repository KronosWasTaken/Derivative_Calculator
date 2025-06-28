use std::collections::HashMap;

use crate::ast::{Expr,Op}; // Assuming `Expr` is in ast.rs

pub fn conversion(func_name: &str, arg: Expr) -> Result<Expr, String> {
    let mut rules: HashMap<&str, &str> = HashMap::new();

    // Regular trigonometric functions
    rules.insert("sin", "cos");
    rules.insert("cos", "-sin");
    rules.insert("tan", "sec^2");
    rules.insert("cot", "-cosec^2");
    rules.insert("sec", "sec*tan");
    rules.insert("cosec", "-cosec*cot");
    
    // Inverse trigonometric functions
    rules.insert("arcsin", "1/sqrt(1-x^2)");
    rules.insert("arccos", "-1/sqrt(1-x^2)");
    rules.insert("arctan", "1/(1+x^2)");
    rules.insert("arccsc", "-1/(|x|*sqrt(x^2-1))");
    rules.insert("arcsec", "1/(|x|*sqrt(x^2-1))");
    rules.insert("arccot", "-1/(1+x^2)");
    
    // Logarithmic and exponential functions
    rules.insert("log", "1/");
    rules.insert("exp", "exp");
    
    // Additional functions needed for inverse trig derivatives
    rules.insert("sqrt", "1/(2*sqrt)");
    rules.insert("abs", "x/|x|");

    match rules.get(func_name) {
        // Regular trigonometric derivatives
        Some(&"cos") => Ok(Expr::Func("cos".to_string(), Box::new(arg))),
        Some(&"-sin") => Ok(Expr::BinaryOp {
            op: Op::Mul,
            left: Box::new(Expr::Var("-1".to_string())),
            right: Box::new(Expr::Func("sin".to_string(), Box::new(arg))),
        }),
        Some(&"sec^2") => Ok(Expr::BinaryOp {
            op: Op::Pow,
            left: Box::new(Expr::Func("sec".to_string(), Box::new(arg.clone()))),
            right: Box::new(Expr::Var("2".to_string())),
        }),
        Some(&"-cosec^2") => Ok(Expr::BinaryOp {
            op: Op::Mul,
            left: Box::new(Expr::Var("-1".to_string())),
            right: Box::new(Expr::BinaryOp {
                op: Op::Pow,
                left: Box::new(Expr::Func("cosec".to_string(), Box::new(arg.clone()))),
                right: Box::new(Expr::Var("2".to_string())),
            }),
        }),
        Some(&"sec*tan") => Ok(Expr::BinaryOp {
            op: Op::Mul,
            left: Box::new(Expr::Func("sec".to_string(), Box::new(arg.clone()))),
            right: Box::new(Expr::Func("tan".to_string(), Box::new(arg))),
        }),
        Some(&"-cosec*cot") => Ok(Expr::BinaryOp {
            op: Op::Mul,
            left: Box::new(Expr::Var("-1".to_string())),
            right: Box::new(Expr::BinaryOp {
                op: Op::Mul,
                left: Box::new(Expr::Func("cosec".to_string(), Box::new(arg.clone()))),
                right: Box::new(Expr::Func("cot".to_string(), Box::new(arg))),
            }),
        }),
        
        // Inverse trigonometric derivatives
        Some(&"1/sqrt(1-x^2)") => Ok(Expr::BinaryOp {
            op: Op::Div,
            left: Box::new(Expr::Var("1".to_string())),
            right: Box::new(Expr::Func("sqrt".to_string(), Box::new(Expr::BinaryOp {
                op: Op::Sub,
                left: Box::new(Expr::Var("1".to_string())),
                right: Box::new(Expr::BinaryOp {
                    op: Op::Pow,
                    left: Box::new(arg),
                    right: Box::new(Expr::Var("2".to_string())),
                }),
            }))),
        }),
        Some(&"-1/sqrt(1-x^2)") => Ok(Expr::BinaryOp {
            op: Op::Mul,
            left: Box::new(Expr::Var("-1".to_string())),
            right: Box::new(Expr::BinaryOp {
                op: Op::Div,
                left: Box::new(Expr::Var("1".to_string())),
                right: Box::new(Expr::Func("sqrt".to_string(), Box::new(Expr::BinaryOp {
                    op: Op::Sub,
                    left: Box::new(Expr::Var("1".to_string())),
                    right: Box::new(Expr::BinaryOp {
                        op: Op::Pow,
                        left: Box::new(arg),
                        right: Box::new(Expr::Var("2".to_string())),
                    }),
                }))),
            }),
        }),
        Some(&"1/(1+x^2)") => Ok(Expr::BinaryOp {
            op: Op::Div,
            left: Box::new(Expr::Var("1".to_string())),
            right: Box::new(Expr::BinaryOp {
                op: Op::Add,
                left: Box::new(Expr::Var("1".to_string())),
                right: Box::new(Expr::BinaryOp {
                    op: Op::Pow,
                    left: Box::new(arg),
                    right: Box::new(Expr::Var("2".to_string())),
                }),
            }),
        }),
        Some(&"-1/(1+x^2)") => Ok(Expr::BinaryOp {
            op: Op::Mul,
            left: Box::new(Expr::Var("-1".to_string())),
            right: Box::new(Expr::BinaryOp {
                op: Op::Div,
                left: Box::new(Expr::Var("1".to_string())),
                right: Box::new(Expr::BinaryOp {
                    op: Op::Add,
                    left: Box::new(Expr::Var("1".to_string())),
                    right: Box::new(Expr::BinaryOp {
                        op: Op::Pow,
                        left: Box::new(arg),
                        right: Box::new(Expr::Var("2".to_string())),
                    }),
                }),
            }),
        }),
        Some(&"-1/(|x|*sqrt(x^2-1))") => Ok(Expr::BinaryOp {
            op: Op::Mul,
            left: Box::new(Expr::Var("-1".to_string())),
            right: Box::new(Expr::BinaryOp {
                op: Op::Div,
                left: Box::new(Expr::Var("1".to_string())),
                right: Box::new(Expr::BinaryOp {
                    op: Op::Mul,
                    left: Box::new(Expr::Func("abs".to_string(), Box::new(arg.clone()))),
                    right: Box::new(Expr::Func("sqrt".to_string(), Box::new(Expr::BinaryOp {
                        op: Op::Sub,
                        left: Box::new(Expr::BinaryOp {
                            op: Op::Pow,
                            left: Box::new(arg),
                            right: Box::new(Expr::Var("2".to_string())),
                        }),
                        right: Box::new(Expr::Var("1".to_string())),
                    }))),
                }),
            }),
        }),
        Some(&"1/(|x|*sqrt(x^2-1))") => Ok(Expr::BinaryOp {
            op: Op::Div,
            left: Box::new(Expr::Var("1".to_string())),
            right: Box::new(Expr::BinaryOp {
                op: Op::Mul,
                left: Box::new(Expr::Func("abs".to_string(), Box::new(arg.clone()))),
                right: Box::new(Expr::Func("sqrt".to_string(), Box::new(Expr::BinaryOp {
                    op: Op::Sub,
                    left: Box::new(Expr::BinaryOp {
                        op: Op::Pow,
                        left: Box::new(arg),
                        right: Box::new(Expr::Var("2".to_string())),
                    }),
                    right: Box::new(Expr::Var("1".to_string())),
                }))),
            }),
        }),
        
        // Additional function derivatives
        Some(&"1/(2*sqrt)") => Ok(Expr::BinaryOp {
            op: Op::Div,
            left: Box::new(Expr::Var("1".to_string())),
            right: Box::new(Expr::BinaryOp {
                op: Op::Mul,
                left: Box::new(Expr::Var("2".to_string())),
                right: Box::new(Expr::Func("sqrt".to_string(), Box::new(arg))),
            }),
        }),
        Some(&"x/|x|") => Ok(Expr::BinaryOp {
            op: Op::Div,
            left: Box::new(arg.clone()),
            right: Box::new(Expr::Func("abs".to_string(), Box::new(arg))),
        }),
        
        // Logarithmic and exponential derivatives
        Some(&"1/") => Ok(Expr::BinaryOp {
            op: Op::Div,
            left: Box::new(Expr::Var("1".to_string())),
            right: Box::new(arg),
        }),
        Some(&"exp") => {
             Ok(Expr::Func("exp".to_string(), Box::new(arg)))

        },
        _ => Err(format!("Unknown function: {}", func_name)),
    }
}
