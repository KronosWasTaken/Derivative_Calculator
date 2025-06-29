mod ast;
mod tokenizer;
mod parser;
mod derivative;
mod function_table;
mod simplifier;
mod constants;

#[tauri::command]
 fn find_der(input_expr: &str,diff_var:&str) -> Result<String, String> {
    let trimmed_expr = input_expr.trim();
    print!("{}",trimmed_expr);
    print!("{}",diff_var);
  

    if trimmed_expr.is_empty() {
        return Err("Input expression is empty".to_string());
    }

    if diff_var.len() != 1 || !diff_var.chars().all(|c| c.is_alphabetic()) {
        return Err("Variable must be a single alphabetic character".to_string());
    }

    // Tokenize expression
    let tokens = tokenizer::tokenize(trimmed_expr)
        .map_err(|e| format!("Tokenizer error: {}", e))?;

    // Parse tokens
    let mut parser = parser::Parser::new(tokens);
    let expr = parser.parse()
        .map_err(|e| format!("Parser error: {}", e))?;

    // Derivative
    let der = derivative::derivative(&expr, diff_var);

    // Simplify
    let simp = simplifier::simplify(&der);
     print!("{}",simp);
    Ok(format!("{}", simp))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![find_der])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
