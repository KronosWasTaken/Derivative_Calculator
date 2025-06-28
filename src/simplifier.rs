use crate::ast::{Expr, Op};



fn is_like_term(a: &Expr, b: &Expr) -> bool {
    match (a, b) {
        (Expr::Var(va), Expr::Var(vb)) => va == vb,
        (Expr::BinaryOp { op: Op::Pow, left: la, right: ra },
         Expr::BinaryOp { op: Op::Pow, left: lb, right: rb }) => la == lb && ra == rb,
        (Expr::Func(fa, aa), Expr::Func(fb, ab)) => fa == fb && aa == ab,
        _ => false,
    }
}


fn flatten_mul(expr: &Expr) -> (f64, Expr) {
    match expr {
        Expr::BinaryOp { op: Op::Mul, left, right } => {
            let (cl, bl) = flatten_mul(left);
            let (cr, br) = flatten_mul(right);
            let coeff = cl * cr;
            let base = if bl == Expr::Num(1.0) {
                br
            } else if br == Expr::Num(1.0) {
                bl
            } else {
                Expr::BinaryOp {
                    op: Op::Mul,
                    left: Box::new(bl),
                    right: Box::new(br),
                }
            };
            (coeff, base)
        }
        Expr::Num(n) => (*n, Expr::Num(1.0)),
        _ => (1.0, expr.clone()),
    }
}

fn extract_coeff_and_base(expr: &Expr) -> (f64, Expr) {
    flatten_mul(expr)
}




pub fn simplify(expr: &Expr) -> Expr {
    use Expr::*;
    use Op::*;
    match expr {
        Num(n) => Num(*n),
        Var(v) => Var(v.clone()),
        Func(name, arg) => {
            let simp_arg = simplify(arg);
            Func(name.clone(), Box::new(simp_arg))
        }
        BinaryOp { op: Add, left, right } => {
            // Flatten and collect all terms in the sum
            let mut terms = vec![];
            fn collect_terms(e: &Expr, terms: &mut Vec<Expr>) {
                if let Expr::BinaryOp { op: Op::Add, left, right } = e {
                    collect_terms(left, terms);
                    collect_terms(right, terms);
                } else {
                    terms.push(simplify(e));
                }
            }
            collect_terms(left, &mut terms);
            collect_terms(right, &mut terms);
            // Combine like terms
            let mut groups: Vec<(Expr, f64)> = vec![];
            for term in terms {
                let (coeff, base) = extract_coeff_and_base(&term);
                let mut found = false;
                for (b, c) in &mut groups {
                    if is_like_term(&base, b) {
                        *c += coeff;
                        found = true;
                        break;
                    }
                }
                if !found {
                    groups.push((base, coeff));
                }
            }
            // Build the sum
            let mut result: Option<Expr> = None;
            for (base, coeff) in groups {
                let term = if let Expr::Num(1.0) = base {
                    Expr::Num(coeff)
                } else if coeff == 1.0 {
                    base
                } else if coeff == 0.0 {
                    Expr::Num(0.0)
                } else {
                    Expr::BinaryOp {
                        op: Op::Mul,
                        left: Box::new(Expr::Num(coeff)),
                        right: Box::new(base),
                    }
                };
                result = match result {
                    None => Some(term),
                    Some(acc) => Some(Expr::BinaryOp {
                        op: Op::Add,
                        left: Box::new(acc),
                        right: Box::new(term),
                    }),
                };
            }
            result.unwrap_or(Expr::Num(0.0))
        }
        BinaryOp { op, left, right } => {
            let l = simplify(left);
            let r = simplify(right);
            match (op, &l, &r) {
                // Addition handled above
                // Subtraction
                (Sub, x, Num(0.0)) => x.clone(),
                (Sub, Num(a), Num(b)) => Num(a - b),
                // Multiplication
                (Mul, Num(0.0), _) | (Mul, _, Num(0.0)) => Num(0.0),
                (Mul, Num(1.0), x) => x.clone(),
                (Mul, x, Num(1.0)) => x.clone(),
                (Mul, Num(a), Num(b)) => Num(a * b),
                // Division
                (Div, x, Num(1.0)) => x.clone(),
                (Div, Num(a), Num(b)) => Num(a / b),
                // Power
                (Pow, _, Num(0.0)) => Num(1.0),
                (Pow, x, Num(1.0)) => x.clone(),
                (Pow, Num(a), Num(b)) => Num(a.powf(*b)),
                // Default: reconstruct
                _ => BinaryOp {
                    op: op.clone(),
                    left: Box::new(l),
                    right: Box::new(r),
                },
            }
        }
    }
}