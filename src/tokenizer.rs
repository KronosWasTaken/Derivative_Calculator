/// Defines the different types of tokens recognized in the input expression.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Num(f64),               // Numeric literals, e.g. 123 or 4.56
    Var(String),            // Variable names, e.g. x, y, or abc
    Func(String), // Function call with name and tokens representing arguments, e.g. sin(x)
    Plus,                   // '+'
    Minus,                  // '-'
    Mul,                    // '*'
    Div,                    // '/'
    Pow,                    // '^'
    LParen,                 // '('
    RParen,                 // ')'
}

/// Helper function that tokenizes an input string into a vector of tokens.
/// This function processes the string character-by-character and applies rules
/// to identify numbers, variables, functions, operators, and parentheses.
/// It does NOT yet handle implicit powers (e.g., x2 as x^2).
fn tokenize_help(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();              // Accumulates tokens found
    let mut chars = input.chars().peekable(); // Peekable iterator for lookahead
    let mut last_token_was_operand = false;  // Tracks if previous token was a number/variable/func (for implicit multiplication)
    let parser_functions = ["sin", "cos", "tan", "cosec", "sec", "cot", "log","exp"]; // Supported functions

    while let Some(&c) = chars.peek() {
        match c {
            // Handle numeric literals, including decimals
            '0'..='9' | '.' => {
                let mut num_str = String::new();
                // Accumulate digits and decimal points
                while let Some(&c) = chars.peek() {
                    if c.is_digit(10) || c == '.' {
                        num_str.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                // Insert implicit multiplication if previous token was operand (e.g., "3x" becomes "3 * x")
                if last_token_was_operand {
                    tokens.push(Token::Mul);
                }
                // Parse collected string as a floating point number
                tokens.push(Token::Num(num_str.parse().map_err(|e| format!("Invalid number: {}", e))?));
                last_token_was_operand = true;
            }
            // Handle variables and function names (alphabetic strings)
            'a'..='z' | 'A'..='Z' => {
                let mut var_str = String::new();
                // Accumulate consecutive alphabetic characters
                while let Some(&c) = chars.peek() {
                    if c.is_alphabetic() {
                        var_str.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                // Insert implicit multiplication if last token was operand (e.g., "3x")
                if last_token_was_operand {
                    tokens.push(Token::Mul);
                }
                // Check if the string is a recognized function name
                if parser_functions.contains(&var_str.as_str()) {
        
                       tokens.push(Token::Func(var_str));
                        last_token_was_operand = true;
                    } 
                 else {
                    // Just a regular variable
                    tokens.push(Token::Var(var_str));
                    last_token_was_operand = true;
                }
            }
            // Handle operators '+', '-', '*', '/', '^'
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
            // Handle parentheses, inserting implicit multiplication if needed (e.g., "3(x+1)")
            '(' => {
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
            // Skip whitespace characters
            ' ' | '\t' | '\n' | '\r' => {
                chars.next();
            }
            // Unrecognized characters cause an error
            _ => return Err(format!("Unknown character: {}", c)),
        }
    }

    Ok(tokens)
}

/// The main tokenizer function which calls `tokenize_help` and additionally
/// processes implicit power expressions, e.g. interpreting "x2" as "x^2".
pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let tokens = tokenize_help(input)?; // First pass tokenization

    let mut final_tokens = Vec::new();
    let mut i = 0;

    while i < tokens.len() {
        final_tokens.push(tokens[i].clone());

        // Detect implicit power: if a Var is immediately followed by a Num,
        // and not already followed by an explicit Pow token,
        // insert a Pow token between them.
        if let Token::Var(_) = tokens[i] {
            if let Some(Token::Num(_)) = tokens.get(i + 1) {
                if tokens.get(i + 2) != Some(&Token::Pow) {
                    final_tokens.push(Token::Pow);
                }
            }
        }

        i += 1;
    }

    Ok(final_tokens)
}
