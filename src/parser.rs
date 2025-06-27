use crate::ast::{Expr, Op};
use crate::tokenizer::Token;

/// The `Parser` takes a sequence of tokens and builds an Abstract Syntax Tree (AST),
/// which represents the mathematical structure of the expression.
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize, // Tracks the parser's current position in the token stream.
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
                self.consume(); // consume the function token
                // Look ahead for function power: e.g., sin^2(x) or sin^2 x or log^2(x)
                let is_power = if let (Some(Token::Pow), Some(Token::Num(_))) = (self.peek(), self.tokens.get(self.pos + 1)) {
                    true
                } else {
                    false
                };
                if is_power {
                    self.consume(); // consume '^'
                    let power_val = if let Some(Token::Num(n)) = self.consume() {
                        *n
                    } else {
                        return Err("Expected number after '^' in function power".to_string());
                    };
                    // Now parse the argument (parenthesis or variable/function/number)
                    let arg = if let Some(Token::LParen) = self.peek() {
                        self.consume(); // consume '('
                        let arg_expr = self.parse_expr()?;
                        match self.consume() {
                            Some(Token::RParen) => arg_expr,
                            _ => return Err("Expected ')' after function argument".to_string()),
                        }
                    } else if matches!(self.peek(), Some(Token::Var(_)) | Some(Token::Num(_)) | Some(Token::Func(_))) {
                        self.parse_primary()?
                    } else {
                        return Err("Expected function argument after power".to_string());
                    };
                    let func_expr = Expr::Func(name.clone(), Box::new(arg));
                    Ok(Expr::BinaryOp {
                        op: Op::Pow,
                        left: Box::new(func_expr),
                        right: Box::new(Expr::Num(power_val)),
                    })
                } else {
                    // Robust function application: exp(x) or exp x
                    let arg = if let Some(Token::LParen) = self.peek() {
                        self.consume(); // consume '('
                        let arg_expr = self.parse_expr()?;
                        match self.consume() {
                            Some(Token::RParen) => arg_expr,
                            _ => return Err("Expected ')' after function argument".to_string()),
                        }
                    } else if matches!(self.peek(), Some(Token::Var(_)) | Some(Token::Num(_)) | Some(Token::Func(_))) {
                        self.parse_primary()?
                    } else {
                        return Err("Expected function argument after function name".to_string());
                    };
                    Ok(Expr::Func(name.clone(), Box::new(arg)))
                }
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
        let mut left = self.parse_primary()?;
        // Handle implicit multiplication: e.g., 2sinx, xsinx, (x+1)sinx
        loop {
            match self.peek() {
                Some(Token::Func(_)) | Some(Token::Var(_)) | Some(Token::Num(_)) | Some(Token::LParen) => {
                    // For function, parse as a primary (which will consume its argument)
                    let right = self.parse_primary()?;
                    left = Expr::BinaryOp {
                        op: Op::Mul,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
                Some(Token::Pow) => {
                    self.consume(); // Consume the '^' token.
                    let right = self.parse_factor()?;
                    left = Expr::BinaryOp {
                        op: Op::Pow,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
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
}

