/// Defines the types of tokens that can be identified within an expression string.
/// These tokens are the fundamental units that the parser will process.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Num(f64),       // A number, like 123 or 4.5
    Var(String),    // A variable, like 'x'
    Plus,           // '+'
    Minus,          // '-'
    Mul,            // '*'
    Div,            // '/'
    Pow,            // '^'
    LParen,         // '('
    RParen,         // ')'
}

/// Transform input string into a sequence (vector) of `Token`s.
pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    // A peekable iterator allows looking at the next character without advancing the position.
    let mut chars = input.chars().peekable();
    // This flag tracks whether the last token was an operand to help detect implicit multiplication (e.g., "3x").
    let mut last_token_was_operand = false;

    while let Some(&c) = chars.peek() {
        match c {
            // Matches a numeric literal, which may include a decimal point.
            '0'..='9' | '.' => {
                let mut num_str = String::new();
                // Consumes consecutive characters that form part of the number.
                while let Some(&c) = chars.peek() {
                    if c.is_digit(10) || c == '.' {
                        num_str.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                // If an operand (a number or variable) was just seen, insert a multiplication
                // token to handle implicit multiplication.
                if last_token_was_operand {
                    tokens.push(Token::Mul);
                }
                // Parses the collected number string into an f64 and creates a `Num` token.
                tokens.push(Token::Num(num_str.parse().map_err(|e| format!("Invalid number: {}", e))?));
                last_token_was_operand = true;
            }
            // Matches a variable name composed of alphabetic characters.
            'a'..='z' | 'A'..='Z' => {
                let mut var_str = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_alphabetic() {
                        var_str.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                // Handles implicit multiplication, like when a variable is followed by another operand.
                if last_token_was_operand {
                    tokens.push(Token::Mul);
                }
                tokens.push(Token::Var(var_str));
                last_token_was_operand = true;
            }
            // Matches standard arithmetic operators.
            '+' => {
                tokens.push(Token::Plus);
                chars.next();
                last_token_was_operand = false;
            }
            '-' => {
                tokens.push(Token::Minus);
                chars.next();
                last_token_was_operand = false;
            }
            '*' => {
                tokens.push(Token::Mul);
                chars.next();
                last_token_was_operand = false;
            }
            '/' => {
                tokens.push(Token::Div);
                chars.next();
                last_token_was_operand = false;
            }
            '^' => {
                tokens.push(Token::Pow);
                chars.next();
                last_token_was_operand = false;
            }
            // Handles opening and closing parentheses.
            '(' => {
                // Handles implicit multiplication before a parenthesis, as in "3(x+1)".
                if last_token_was_operand {
                    tokens.push(Token::Mul);
                }
                tokens.push(Token::LParen);
                chars.next();
                last_token_was_operand = false;
            }
            ')' => {
                tokens.push(Token::RParen);
                chars.next();
                last_token_was_operand = true;
            }
            // Skips over any whitespace characters.
            ' ' | '\t' | '\n' | '\r' => {
                chars.next();
            }
            // Returns an error for any unrecognized characters.
            _ => return Err(format!("Unknown character: {}", c)),
        }
    }

    // A second pass is performed to handle implicit powers (e.g., "x2" or "3x2").
    // This is done separately to simplify the main loop. For instance, "x2"
    // must be tokenized as `Var(x), Pow, Num(2)`.
    let mut final_tokens = Vec::new();
    let mut i = 0;
    while i < tokens.len() {
        final_tokens.push(tokens[i].clone());
        // Checks if a variable is immediately followed by a number.
        if let Some(Token::Var(_)) = tokens.get(i) {
            if let Some(Token::Num(_)) = tokens.get(i+1) {
                // ...and if it's not already part of an explicit power operation...
                if i+2 >= tokens.len() || tokens.get(i+2) != Some(&Token::Pow) {
                     // ...inserts a `Pow` token to represent the implicit power.
                     final_tokens.push(Token::Pow);
                }
            }
        }
        i += 1;
    }
    Ok(final_tokens)
} 