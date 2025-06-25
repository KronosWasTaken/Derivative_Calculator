use std::io::{self, Write};

// Declares the modules that the application uses.
// This tells the Rust compiler to look for `ast.rs`, `tokenizer.rs`, and `parser.rs`.
mod ast;
mod tokenizer;
mod parser;

/// The main entry point for the application.
fn main() {
    // Starts an infinite loop to create a Read-Eval-Print Loop (REPL).
    loop {
        // Displays the prompt to the user.
        print!("Enter string: ");
        // `print!` is buffered, so we flush stdout to ensure the prompt appears
        // before the program waits for input.
        io::stdout().flush().unwrap();

        let mut input = String::new();
        // Reads a line of input from the user.
        // If `read_line` returns 0 bytes, it signifies the end of the input stream
        // (e.g., from Ctrl+D on Unix or Ctrl+Z on Windows), so we exit the loop.
        if io::stdin().read_line(&mut input).unwrap() == 0 {
            println!(); // Prints a newline for a clean exit.
            break; 
        }

        // Removes any leading or trailing whitespace from the user's input.
        let trimmed_input = input.trim();

        // If the input is empty, restart the loop to wait for the next input.
        if trimmed_input.is_empty() {
            continue;
        }

        // Allows the user to exit the program by typing "exit".
        if trimmed_input == "exit" {
            break;
        }
        
        // This block contains the core logic of the REPL:
        // 1. The input string is passed to the tokenizer.
        // 2. If tokenization succeeds, a new parser is created with the tokens.
        // 3. The parser attempts to build an AST from the tokens.
        // 4. The resulting AST or any errors are printed to the console.
        match tokenizer::tokenize(trimmed_input) {
            Ok(tokens) => {
                let mut parser = parser::Parser::new(tokens);
                match parser.parse() {
                    Ok(expr) => println!("{}", expr),
                    Err(e) => eprintln!("Parser error: {}", e),
                }
            }
            Err(e) => eprintln!("Tokenizer error: {}", e),
        }
    }
}
