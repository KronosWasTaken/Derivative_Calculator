use std::io::{self, Write};

mod ast;
mod tokenizer;
mod parser;
mod derivative;
mod function_table;






fn main() {
    loop {
        print!("Enter string: ");
        io::stdout().flush().unwrap(); // Still safe to unwrap here

        let mut input = String::new();
        
        // Handle stdin reading errors explicitly
        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                // 0 bytes read means EOF (Ctrl+D / Ctrl+Z)
                println!();
                break;
            }
            Ok(_) => {
                let trimmed_input = input.trim();

                if trimmed_input.is_empty() {
                    continue;
                }

                if trimmed_input == "exit" {
                    break;
                }

                // Tokenize and parse with error handling
                match tokenizer::tokenize(trimmed_input) {
                    Ok(tokens) => {
                        let mut parser = parser::Parser::new(tokens);
                        match parser.parse() {
                            Ok(expr) =>{
                                println!("Printing the expression");
                                println!("{}",expr);
                                let der=derivative::derivative(&expr, "x");
                                println!("derivative:{}",der);
                              
                         
        
                            },
                            Err(e) => eprintln!("Parser error: {}", e),
                        }
                    }
                    Err(e) => eprintln!("Tokenizer error: {}", e),
                }
            }
            Err(e) => {
                // Graceful handling of IO read error
                eprintln!("Input error: {}", e);
                break;
            }
        }
    }
}
