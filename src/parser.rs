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
        match self.consume() {
            Some(Token::Num(n)) => Ok(Expr::Num(*n)),
            Some(Token::Var(s)) => Ok(Expr::Var(s.clone())),
    Some(Token::Func(name)) => {
    let func_name = name.clone();
    self.consume(); // consume the Func token

    match self.peek() {
        Some(Token::LParen) => {
            self.consume(); // consume '('
            let arg_expr = self.parse_expr()?;
            match self.consume() {
                Some(Token::RParen) => Ok(Expr::Func(func_name, Box::new(arg_expr))),
                _ => Err("Expected ')' after function argument".to_string()),
            }
        }

        Some(Token::Pow) => {
            self.consume(); // consume '^'
            let exponent = self.parse_factor()?; // e.g., Num(2)

            match self.consume() {
                Some(Token::LParen) => {
                    let mut depth = 1;
                    let mut arg_tokens = Vec::new();

                    while let Some(tok) = self.consume() {
                        match tok {
                            Token::LParen => {
                                depth += 1;
                                arg_tokens.push(tok.clone());
                            }
                            Token::RParen => {
                                depth -= 1;
                                if depth == 0 {
                                    break;
                                } else {
                                    arg_tokens.push(tok.clone());
                                }
                            }
                            _ => arg_tokens.push(tok.clone()),
                        }
                    }

                    if depth != 0 {
                        return Err("Unmatched parentheses in powered function argument".to_string());
                    }

                    let mut arg_parser = Parser::new(arg_tokens);
                    let arg_expr = arg_parser.parse_expr()?;

                    let func_expr = Expr::Func(func_name, Box::new(arg_expr));

                    Ok(Expr::BinaryOp {
                        op: Op::Pow,
                        left: Box::new(func_expr),
                        right: Box::new(exponent),
                    })
                }
                _ => Err("Expected '(' after powered function name".to_string()),
            }
        }

        _ => Err("Expected '(' or '^' after function name".to_string()),
    }
},

        
          
            Some(Token::LParen) => {
                // If an opening parenthesis is found, recursively call `parse_expr`
                // to handle the nested expression.
                let expr = self.parse_expr()?;
                // A closing parenthesis is then required.
                match self.consume() {
                    Some(Token::RParen) => Ok(expr),
                    _ => Err("Expected ')'".to_string()),
                }
            }
            _ => Err("Expected a number, variable, or '('".to_string()),
        }
    }

    /// Parses exponentiation (the `^` operator). Exponentiation has higher
    /// precedence than multiplication and division and is right-associative,
    /// meaning `2^3^4` is parsed as `2^(3^4)`.
    fn parse_factor(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_primary()?;
        while let Some(Token::Pow) = self.peek() {
            self.consume(); // Consume the '^' token.
            // Recursively call `parse_factor` for the right-hand side to handle
            // right-associativity.
            let right = self.parse_factor()?;
            left = Expr::BinaryOp {
                op: Op::Pow,
                left: Box::new(left),
                right: Box::new(right),
            };
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

