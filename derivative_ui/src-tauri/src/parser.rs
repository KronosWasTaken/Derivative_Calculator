use crate::ast::{Expr, Op};
use crate::tokenizer::Token;

/// The `Parser` takes a sequence of tokens and builds an Abstract Syntax Tree (AST),
/// which represents the mathematical structure of the expression.
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize, // Tracks the parser's current position in the token stream.
}

fn traverse_expr(expr: &Expr) {
    match expr {
        Expr::Num(n) => {
            println!("Num: {}", n);
        }
        Expr::Var(name) => {
            println!("Var: {}", name);
        }
        Expr::BinaryOp { op, left, right } => {
            println!("BinaryOp: {:?}", op);
            traverse_expr(left);
            traverse_expr(right);
        }
        Expr::Func(name, arg) => {
            println!("Func: {}", name);
            traverse_expr(arg);
        }
    }
}









impl Parser {
    /// Creates a new `Parser` for a given list of tokens.
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    /// The primary public method that initiates the parsing process.
    /// It expects to parse a complete expression and returns an error
    /// if any tokens remain unconsumed.
    pub fn parse(&mut self) -> Result<Expr, String> {
        let expr = self.parse_expr()?;
        // After a successful parse, we expect to be at the end of the token stream.
        if self.pos < self.tokens.len() {
            return Err("Unexpected token at end of expression".to_string());
        }
        Ok(expr)
    }

    /// Returns a reference to the current token without consuming it.
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    /// Consumes the current token and advances the parser's position in the stream.
    fn consume(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.pos);
        self.pos += 1;
        token
    }

    /// Parses the most fundamental units of an expression: numbers, variables,
    /// or sub-expressions enclosed in parentheses. This function handles the
    /// highest level of operator precedence.
    fn parse_primary(&mut self) -> Result<Expr, String> {
        // Look ahead at the next token without consuming
        let next_token = self.peek().cloned();
        match next_token {
            Some(Token::Num(n)) => {
                self.consume();
                Ok(Expr::Num(n))
            },
            Some(Token::Var(ref s)) => {
                self.consume();
                Ok(Expr::Var(s.clone()))
            },
            
           Some(Token::Func(ref name)) => {
    self.consume(); // consume the function token, e.g., "sin"

    // Check for power operator '^' right after function name
    let power_expr = if matches!(self.peek(), Some(Token::Pow))
     {
    self.consume(); // consume '^'

    if let Some(Token::Num(n)) = self.peek() {
        let n_val = *n;
        self.consume(); // consume number token
       
        Expr::Num(n_val)
    } else if let Some(Token::LParen) = self.peek() {
     
        self.consume(); // consume '('
        println!("{:?} checking",self.peek());
        let inner = self.parse_expr()?;  // parse inner expression
        match self.consume() {
            Some(Token::RParen) => {
                self.consume();
                inner
            },
            _ => return Err("Expected ')' after power expression".to_string()),
        }
    } else {
        self.parse_exponent_chain()?
    }
} else {
    // No power '^' found, so power is implicitly 1
    Expr::Num(1.0)
};






    // Now parse the function argument AFTER the power expression
    let arg = if matches!(self.peek(), Some(Token::LParen)) {
        self.consume(); // consume '('
        println!("{:?}",self.peek());
        let arg_expr = self.parse_expr()?;
        match self.consume() {
            Some(Token::RParen) => arg_expr,
            _ => return Err("Expected ')' after function argument".to_string()),
        }
    }else if matches!(self.peek(),Some(Token::Var(_)) | Some(Token::Num(_)) | Some(Token::Func(_)  )) {
        println!("{:?} peeking",self.peek());
    
        self.parse_primary()?
    } else {
        return Err("Expected function argument after power".to_string());
    };

  traverse_expr(&arg);
    



    // Build the function expression: sin(x) or sin^(power)(x)
    let func_expr = Expr::Func(name.clone(), Box::new(arg));

    // If power was 1, just return the function call
    if let Expr::Num(n) = power_expr {
        if (n - 1.0).abs() < std::f64::EPSILON {
            return Ok(func_expr);
        }
    }

    // Otherwise, return power expression: (sin(x))^(power_expr)



    Ok(Expr::BinaryOp {
        op: Op::Pow,
        left: Box::new(func_expr),
        right: Box::new(power_expr),
    })
},
            Some(Token::LParen) => {
                self.consume(); // consume '('
                let expr = self.parse_expr()?;
                match self.consume() {
                    Some(Token::RParen) => Ok(expr),
                    _ => Err("Expected ')'".to_string()),
                }
            },
            _ => Err("Expected a number, variable, or '('".to_string()),
        }
    }

    /// Parses exponentiation (the `^` operator). Exponentiation has higher
    /// precedence than multiplication and division and is right-associative,
    /// meaning `2^3^4` is parsed as `2^(3^4)`.
    
fn parse_factor(&mut self) -> Result<Expr, String> {
    // Start by parsing unary expressions to handle negation
    let mut left = self.parse_unary()?;

    loop {
        match self.peek() {
            // Implicit multiplication: e.g., 2sinx, xsinx, (x+1)sinx
            Some(Token::Func(_)) | Some(Token::Var(_)) | Some(Token::Num(_)) | Some(Token::LParen) => {
                let right = self.parse_unary()?;
                left = Expr::BinaryOp {
                    op: Op::Mul,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            }

            Some(Token::Pow) => {
                self.consume(); // consume '^'

                match self.peek() {
                    Some(Token::Num(_)) => {
                        // Peek ahead to check if exponent is composite, e.g. 2x
                        if let Some(next_token) = self.tokens.get(self.pos + 1) {
                            if matches!(next_token, Token::Var(_) | Token::Num(_) | Token::Func(_) | Token::LParen) {
                                // Parse complex exponent chain like 2x = 2 * x
                                let right = self.parse_exponent_chain()?;
                                left = Expr::BinaryOp {
                                    op: Op::Pow,
                                    left: Box::new(left),
                                    right: Box::new(right),
                                };
                            } else {
                                // Single number exponent
                                let right = self.parse_factor()?;
                                left = Expr::BinaryOp {
                                    op: Op::Pow,
                                    left: Box::new(left),
                                    right: Box::new(right),
                                };
                            }
                        } else {
                            // End of tokens, just parse factor
                            let right = self.parse_factor()?;
                            left = Expr::BinaryOp {
                                op: Op::Pow,
                                left: Box::new(left),
                                right: Box::new(right),
                            };
                        }
                    }

                    Some(Token::Var(_)) | Some(Token::Func(_)) | Some(Token::LParen) => {
                        // Exponent is variable, function call, or parenthesis expression
                        let right = self.parse_exponent_chain()?;
                        left = Expr::BinaryOp {
                            op: Op::Pow,
                            left: Box::new(left),
                            right: Box::new(right),
                        };
                    }

                    _ => {
                        return Err("Unexpected token after '^'".to_string());
                    }
                }
            }

            _ => break,
        }
    }

    Ok(left)
}




    /// Parses multiplication and division operators (`*`, `/`). This precedence
    /// level is below exponentiation but above addition and subtraction.
    fn parse_term(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_factor()?;
        while let Some(token) = self.peek() {
            match token {
                Token::Mul | Token::Div => {
                    let op = if matches!(token, Token::Mul) { Op::Mul } else { Op::Div };
                    self.consume(); // Consume the '*' or '/' token.
                    let right = self.parse_factor()?;
                    left = Expr::BinaryOp {
                        op,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }
        Ok(left)
    }

    /// Parses addition and subtraction operators (`+`, `-`), which have the
    /// lowest level of operator precedence.
    fn parse_expr(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_term()?;
        while let Some(token) = self.peek() {
            match token {
                Token::Plus | Token::Minus => {
                    let op = if matches!(token, Token::Plus) { Op::Add } else { Op::Sub };
                    self.consume(); // Consume the '+' or '-' token.
                    let right = self.parse_term()?;
                    left = Expr::BinaryOp {
                        op,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }
        Ok(left)
    }
/// Parse exponent expression after '^', supporting implicit multiplication.
/// e.g., sin^2x → sin^(2 * x)
fn parse_exponent_chain(&mut self) -> Result<Expr, String> {
    // first part: could be number, var, function, or parenthesized expr
    let mut expr = if let Some(Token::LParen) = self.peek() {
        self.consume(); // consume '('
        let inner = self.parse_expr()?;
        match self.consume() {
            Some(Token::RParen) => inner,
            _ => return Err("Expected ')' after exponent expression".to_string()),
        }
    } else {
        self.parse_primary()?
    };

    // Keep multiplying by next parts if there's implicit multiplication:
    loop {
        match self.peek() {
            Some(Token::Var(_)) | Some(Token::Num(_)) | Some(Token::Func(_)) | Some(Token::LParen) => {
                let next = self.parse_primary()?;
                expr = Expr::BinaryOp {
                    op: Op::Mul,
                    left: Box::new(expr),
                    right: Box::new(next),
                };
            }
            _ => break,
        }
    }

    Ok(expr)
}

fn parse_unary(&mut self) -> Result<Expr, String> {
    if let Some(Token::Minus) = self.peek() {
        self.consume(); // consume '-'
        let expr = self.parse_unary()?;  // recursively parse after minus (support --x)
        match expr {
            Expr::Num(n) => Ok(Expr::Num(-n)),  // Negate number literals directly
            _ => {
                // For non-numbers, represent unary minus as multiplication by -1
                Ok(Expr::BinaryOp {
                    op: Op::Mul,
                    left: Box::new(Expr::Num(-1.0)),
                    right: Box::new(expr),
                })
            }
        }
    } else {
        self.parse_primary()  // fallback to primary parsing
    }
}






}
