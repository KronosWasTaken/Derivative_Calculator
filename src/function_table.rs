use std::collections::HashMap;

use crate::ast::{Expr,Op}; // Assuming `Expr` is in ast.rs

pub fn conversion(func_name: &str, arg: Expr) -> Result<Expr, String> {
    let mut rules: HashMap<&str, &str> = HashMap::new();

    rules.insert("sin", "cos");
    rules.insert("cos", "-sin");
    rules.insert("tan", "sec^2");
    rules.insert("cot", "-cosec^2");
    rules.insert("sec", "sec*tan");
    rules.insert("cosec", "-cosec*cot");
    rules.insert("log", "1/");
    rules.insert("exp", "exp");

    match rules.get(func_name) {
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
