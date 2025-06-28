use std::io::{self, Write};

mod ast;
mod tokenizer;
mod parser;
mod derivative;
mod function_table;
mod simplifier;

fn print_manual() {
    println!("\n=== Derivative Calculator Manual ===");
    println!("You can input expressions using the following syntax:");
    println!("  - sin(x)         : Sine of x");
    println!("  - cos(x)         : Cosine of x");
    println!("  - tan(x)         : Tangent of x");
    println!("  - exp(x)         : Exponential function, e^x");
    println!("  - log(x)         : Natural logarithm (ln(x))");
    println!("  - sin^2(x)       : (sin(x))^2");
    println!("  - cos^3 x        : (cos(x))^3");
    println!("  - 2sinx          : 2 * sin(x)");
    println!("  - x^3 + 2x + 1   : Polynomial");
    println!("  - (x+1)*(x-1)    : Parentheses for grouping");
    println!("  - exit           : Quit the program");
    println!("\nTips:");
    println!("- You can use implicit multiplication: 2x means 2*x, sin2x means sin(2*x)");
    println!("- You can use powers on functions: sin^2(x) means (sin(x))^2");
    println!("- You can use parentheses for clarity: sin^2(x+1)");
    println!("- Supported functions: sin, cos, tan, exp, log, etc.\n");
}

fn main() {
    loop {
        println!("\n=== Derivative Calculator ===");
        println!("1. Show manual");
        println!("2. Enter string");
        println!("3. Exit");
        print!("Choose an option (1, 2, or 3): ");
        io::stdout().flush().unwrap();
        let mut choice = String::new();
        if io::stdin().read_line(&mut choice).unwrap() == 0 {
            println!();
            break;
        }
        let choice = choice.trim();
        if choice == "1" {
            print_manual();
            continue;
        } else if choice == "2" {
            break;
        } else if choice == "3" || choice.eq_ignore_ascii_case("exit") {
            println!("Exiting Derivative Calculator");
            return;
        } else {
            println!("Invalid option. Please enter 1, 2, or 3.");
        }
    }

    loop {
    print!("Enter expression string (or 'exit' to quit): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(0) => {
            println!();
            break;
        }
        Ok(_) => {
            let trimmed_input = input.trim();

            if trimmed_input.is_empty() {
                continue;
            }

            if trimmed_input.eq_ignore_ascii_case("exit") {
                break;
            }

            // Now ask for the variable to differentiate with respect to
            let var = loop {
                print!("Differentiate with respect to variable (e.g., x): ");
                io::stdout().flush().unwrap();

                let mut var_input = String::new();
                match io::stdin().read_line(&mut var_input) {
                    Ok(0) => {
                        println!("\nNo input detected, exiting.");
                        return;
                    }
                    Ok(_) => {
                        let var_trimmed = var_input.trim();
                        if var_trimmed.len() == 1 && var_trimmed.chars().all(|c| c.is_alphabetic()) {
                            break var_trimmed.to_string();
                        } else {
                            println!("Please enter a single alphabetic character for the variable.");
                        }
                    }
                    Err(e) => {
                        eprintln!("Input error: {}", e);
                        return;
                    }
                }
            };

            // Tokenize and parse with error handling
            match tokenizer::tokenize(trimmed_input) {
                Ok(tokens) => {
                    let mut parser = parser::Parser::new(tokens);
                    match parser.parse() {
                        Ok(expr) => {
                            println!("Parsed expression:\n{}", expr);
                            let der = derivative::derivative(&expr, &var);
                            let simp = simplifier::simplify(&der);
                            println!("Derivative with respect to '{}':\n{}", var, simp);
                        }
                        Err(e) => eprintln!("Parser error: {}", e),
                    }
                }
                Err(e) => eprintln!("Tokenizer error: {}", e),
            }
        }
        Err(e) => {
            eprintln!("Input error: {}", e);
            break;
        }
    }
}


    

   
}
